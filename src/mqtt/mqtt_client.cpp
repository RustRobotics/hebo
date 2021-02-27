// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/mqtt_client.h"

#include <mqtt_client_cpp.hpp>

namespace hebo {
namespace {

using InternalClient = std::shared_ptr<
    MQTT_NS::callable_overlay<MQTT_NS::async_client<
        MQTT_NS::tcp_endpoint<as::ip::tcp::socket,
                              as::io_context::strand>>>>;

}  // namespace

struct MqttClientPrivate {
  boost::asio::io_context context{};
  InternalClient client{};
};

MqttClient::MqttClient(QObject* parent)
    : QObject(parent),
      worker_thread_(new QThread(this)),
      p_(new MqttClientPrivate()) {
  this->initSignals();

//  this->worker_thread_->start();
}

MqttClient::~MqttClient() {
  delete this->p_;
}

void MqttClient::initSignals() {
  connect(this->worker_thread_, &QThread::finished,
          this->worker_thread_, &QThread::deleteLater);

  connect(this, &MqttClient::stateChanged, [](ConnectionState state) {
    qDebug() << "state:" << state;
  });
}

void MqttClient::setState(ConnectionState state) {
  this->state_ = state;
  emit this->stateChanged(state);
}

void MqttClient::requestConnect() {
  auto c = MQTT_NS::make_async_client(p_->context,
                                      this->config_.host.toStdString(),
                                      this->config_.port);
  p_->client = c;

  c->set_client_id(this->config_.client_id.toStdString());
  c->set_clean_session(this->config_.clean_session);
  using PacketId = typename std::remove_reference_t<decltype(*c)>::packet_id_t;

  c->set_connack_handler([=](bool sp, MQTT_NS::connect_return_code rc) {
    qDebug() << "sp:" << sp << MQTT_NS::connect_return_code_to_str(rc);
    this->setState(ConnectionConnected);
    emit this->connectResult(!sp, MQTT_NS::connect_return_code_to_str(rc));

    c->async_subscribe("hello", MQTT_NS::qos::exactly_once);

    return true;
  });

  c->set_publish_handler([&](MQTT_NS::optional<PacketId> packet_id,
                             MQTT_NS::publish_options pubopts,
                             MQTT_NS::buffer topic_name,
                             MQTT_NS::buffer contents) {
    std::cout << "publish received."
              << " dup: " << pubopts.get_dup()
              << " qos: " << pubopts.get_qos()
              << " retain: " << pubopts.get_retain() << std::endl;
    if (packet_id) {
      std::cout << "packet_id: " << *packet_id << std::endl;
    }
    std::cout << "topic_name: " << topic_name << std::endl;
    std::cout << "contents: " << contents << std::endl;

    return true;
  });

  c->set_close_handler([&]() {
    qDebug() << __func__ << "close handler";
    this->setState(ConnectionDisconnected);
  });
  c->set_error_handler([&](MQTT_NS::error_code ec) {
    qWarning() << "Got mqtt error:" << ec.message().c_str();
  });

  c->async_connect();
  this->timer_id_ = this->startTimer(5);
  this->setState(ConnectionConnecting);
}

void MqttClient::requestDisconnect() {
  this->setState(ConnectionDisconnecting);
  this->killTimer(this->timer_id_);
}

void MqttClient::timerEvent(QTimerEvent* event) {
  QObject::timerEvent(event);
  this->p_->context.poll();
}

void MqttClient::requestSubscribe(const QString& topic, QoS qos) {
  Q_UNUSED(topic)
  Q_UNUSED(qos)
}

void MqttClient::requestUnsubscribe(const QString& topic) {
  Q_UNUSED(topic);
}

void MqttClient::requestPublish(const QString& topic, int qos, const QByteArray& payload) {
  Q_ASSERT(this->state_ == ConnectionConnected);
  if (this->state_ != ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }
  Q_UNUSED(qos);

  const auto topic_str = topic.toStdString();
  this->p_->client->async_publish(MQTT_NS::allocate_buffer(topic_str),
                                  MQTT_NS::allocate_buffer(payload.constData()),
                                  MQTT_NS::qos::exactly_once, [](MQTT_NS::error_code ec) {
    qWarning() << "ec;" << ec.message().c_str();
  });
}

}  // namespace
