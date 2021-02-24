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
  ConnInfo conn_info{};
  boost::asio::io_context context{};
  InternalClient client{};
  int timer_id{-1};
};

MqttClient::MqttClient(QObject* parent) : QObject(parent), p_(new MqttClientPrivate()) {
  this->initSignals();
}

MqttClient::~MqttClient() {
  delete this->p_;
}

void MqttClient::initSignals() {
  connect(this, &MqttClient::requestConnect,
          this, &MqttClient::doConnect);
  connect(this, &MqttClient::requestDisconnect,
          this, &MqttClient::doDisconnect);
  connect(this, &MqttClient::requestPublish,
          this, &MqttClient::doPublish);
  connect(this, &MqttClient::requestSubscribe,
          this, &MqttClient::doSubscribe);
  connect(this, &MqttClient::requestUnsubscribe,
          this, &MqttClient::doUnsubscribe);
}

void MqttClient::doConnect(const ConnInfo& info) {
  this->p_->conn_info = info;
  this->initClient();
}

void MqttClient::initClient() {
  auto c = MQTT_NS::make_async_client(p_->context, p_->conn_info.host.toStdString(),
                                      p_->conn_info.port);
  p_->client = c;

  c->set_client_id(p_->conn_info.client_id.toStdString());
  c->set_clean_session(p_->conn_info.clean_session);
    using PacketId = typename std::remove_reference_t<decltype(*c)>::packet_id_t;
  c->set_connack_handler([=](bool sp, MQTT_NS::connect_return_code rc) {
    qDebug() << "sp:" << sp << MQTT_NS::connect_return_code_to_str(rc);
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
  });
  c->set_error_handler([&](MQTT_NS::error_code ec) {
    qWarning() << "Got mqtt error:" << ec.message().c_str();
  });

  c->async_connect();
  p_->timer_id = this->startTimer(5);
  qDebug() << __func__ << "timer id:" << p_->timer_id;
}

void MqttClient::doDisconnect() {
//  this->killTimer(p_->timer_id);
}

void MqttClient::timerEvent(QTimerEvent* event) {
  QObject::timerEvent(event);
  this->p_->context.poll();
}

void MqttClient::doSubscribe(const QString& topic, int qos) {
  Q_UNUSED(topic)
  Q_UNUSED(qos)
}

void MqttClient::doUnsubscribe(const QString& topic) {
  Q_UNUSED(topic);
}

void MqttClient::doPublish(const QString& topic, int qos, const QByteArray& payload) {
  Q_UNUSED(topic);
  Q_UNUSED(qos);
  Q_UNUSED(payload);
}

}  // namespace