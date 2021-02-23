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
}

void MqttClient::doConnect(const ConnInfo& info) {
  qDebug() << __func__ << info;
  this->p_->conn_info = info;
  this->initClient();
}

void MqttClient::initClient() {
  auto c = MQTT_NS::make_async_client(p_->context, p_->conn_info.host.toStdString(),
                                      p_->conn_info.port);
  p_->client = c;
  c->set_client_id(p_->conn_info.client_id.toStdString());
  c->set_clean_session(p_->conn_info.clean_session);
  c->set_connack_handler([&](bool sp, MQTT_NS::connect_return_code rc) {
    qDebug() << "sp:" << sp << MQTT_NS::connect_return_code_to_str(rc);
    return true;
  });

  c->async_connect();
}

void MqttClient::doDisconnect() {

}

}  // namespace