// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/internal_client.h"

#include <QDebug>
#include <mqtt_client_cpp.hpp>

namespace hebo {
namespace {

using ClientRef = std::shared_ptr<
    MQTT_NS::callable_overlay<MQTT_NS::async_client<
        MQTT_NS::tcp_endpoint<as::ip::tcp::socket,
    as::io_context::strand>>>>;

}  // namespace

struct MqttClientPrivate {
  boost::asio::io_context context{};
  ClientRef client{};
};

InternalClient::InternalClient(QObject* parent)
    : QObject(parent),
      p_(new MqttClientPrivate()) {
  this->initSignals();
}

InternalClient::~InternalClient() {
  delete this->p_;
}

void InternalClient::initSignals() {
  connect(this, &InternalClient::requestConnect,
          this, &InternalClient::doConnect);
  connect(this, &InternalClient::requestDisconnect,
          this, &InternalClient::doDisconnect);
  connect(this, &InternalClient::requestSubscribe,
          this, &InternalClient::doSubscribe);
  connect(this, &InternalClient::requestUnsubscribe,
          this, &InternalClient::doUnsubscribe);
  connect(this, &InternalClient::requestPublish,
          this, &InternalClient::doPublish);
}

void InternalClient::doConnect(const ConnectConfig& config) {
  auto c = MQTT_NS::make_async_client(p_->context,
                                      config.host.toStdString(),
                                      config.port);
  this->p_->client = c;

  c->set_client_id(config.client_id.toStdString());
  c->set_clean_session(config.clean_session);
  using PacketId = typename std::remove_reference_t<decltype(*c)>::packet_id_t;

  c->set_connack_handler([=](bool sp, MQTT_NS::connect_return_code rc) {
    Q_UNUSED(sp);
    Q_UNUSED(rc);
    emit this->stateChanged(ConnectionConnected);
    return true;
  });

  c->set_publish_handler([&](MQTT_NS::optional<PacketId> packet_id,
                             MQTT_NS::publish_options pubopts,
                             MQTT_NS::buffer topic_name,
                             MQTT_NS::buffer contents) {
    if (packet_id) {
      std::cout << "packet_id: " << *packet_id << std::endl;
    }

    MqttMessage message{};
    message.topic = QString::fromUtf8(topic_name.data(), topic_name.size());
    message.qos = static_cast<QoS>(pubopts.get_qos());
    message.is_publish = false;
    message.payload.append(contents.data(), contents.size());
    emit this->messageReceived(message);

    return true;
  });

  c->set_close_handler([&]() {
    qDebug() << "close handler";
    emit this->stateChanged(ConnectionDisconnected);
    this->killTimer(this->timer_id_);
  });
  c->set_error_handler([&](MQTT_NS::error_code ec) {
    qWarning() << "Got mqtt error:" << ec.message().c_str();
  });

  c->async_connect();
  this->timer_id_ = this->startTimer(5);
  emit this->stateChanged(ConnectionConnecting);
}

void InternalClient::doDisconnect() {
  this->p_->client->async_disconnect([=](MQTT_NS::error_code ec) {
    qDebug() << "async_disconnect() returns:" << ec.message().data();
  });
}

void InternalClient::doSubscribe(const QString& topic, QoS qos) {
  const std::string topic_str = topic.toStdString();
  this->p_->client->async_subscribe(topic_str, static_cast<MQTT_NS::qos>(qos));
}

void InternalClient::doUnsubscribe(const QString& topic) {
  const std::string topic_str = topic.toStdString();
  this->p_->client->async_unsubscribe(topic_str);
}

void InternalClient::doPublish(const QString& topic, QoS qos, const QByteArray& payload) {
  const auto topic_str = topic.toStdString();
  this->p_->client->async_publish(MQTT_NS::allocate_buffer(topic_str),
                                  MQTT_NS::allocate_buffer(payload.constData()),
                                  MQTT_NS::qos::exactly_once, [](MQTT_NS::error_code ec) {
        qWarning() << "ec;" << ec.message().data();
      });
}

}  // namespace hebo