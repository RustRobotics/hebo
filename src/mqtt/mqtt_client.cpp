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
      subscriptions_(new SubscriptionModel(this)),
      messages_(new MessageStreamModel(this)),
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

  connect(this->subscriptions_, &SubscriptionModel::dataChanged, [=]() {
    emit this->subscriptionsChanged(this->subscriptions_);
  });

  connect(this->messages_, &MessageStreamModel::dataChanged, [=]() {
    emit this->messagesChanged(this->messages_);
  });

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
    Q_UNUSED(sp);
    Q_UNUSED(rc);
    this->setState(ConnectionConnected);
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

    MqttMessage message{};
    message.topic.append(topic_name.data());
    message.qos = static_cast<QoS>(pubopts.get_qos());
    message.is_publish = false;
    message.payload.append(contents.data(), contents.size());
    this->messages_->addMessage(message);

    return true;
  });

  c->set_close_handler([&]() {
    qDebug() << "close handler";
    this->setState(ConnectionDisconnected);
    this->killTimer(this->timer_id_);
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
  this->p_->client->async_disconnect([=](MQTT_NS::error_code ec) {
    qDebug() << "async_disconnect() returns:" << ec.message().data();
  });
}

void MqttClient::timerEvent(QTimerEvent* event) {
  QObject::timerEvent(event);
  this->p_->context.poll();
}

void MqttClient::requestSubscribe(const QString& topic, int qos, const QString& color) {
  qDebug() << __func__ << topic;

  Q_ASSERT(this->state_ == ConnectionConnected);
  if (this->state_ != ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }

  if (this->subscriptions_->hasSubscription(topic)) {
    qWarning() << "Topic already subscribed:" << topic;
    return;
  }

  this->subscriptions_->addSubscription(topic, qos, color);
  const std::string topic_str = topic.toStdString();
  this->p_->client->async_subscribe(topic_str, static_cast<MQTT_NS::qos>(qos));
}

void MqttClient::requestUnsubscribe(const QString& topic) {
  Q_ASSERT(this->state_ == ConnectionConnected);
  if (this->state_ != ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }

  if (this->subscriptions_->removeSubscription(topic)) {
    const std::string topic_str = topic.toStdString();
    this->p_->client->async_unsubscribe(topic_str);
  } else {
    qWarning() << "Topic with name not subscribed:" << topic;
  }
}

void MqttClient::requestPublish(const QString& topic, int qos, const QByteArray& payload) {
  Q_ASSERT(this->state_ == ConnectionConnected);
  if (this->state_ != ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }

  const auto topic_str = topic.toStdString();
  this->p_->client->async_publish(MQTT_NS::allocate_buffer(topic_str),
                                  MQTT_NS::allocate_buffer(payload.constData()),
                                  MQTT_NS::qos::exactly_once, [](MQTT_NS::error_code ec) {
    qWarning() << "ec;" << ec.message().data();
  });

  MqttMessage message{};
  message.topic = topic;
  message.qos = static_cast<QoS>(qos);
  message.is_publish = true;
  message.payload = payload;
  this->messages_->addMessage(message);
}

}  // namespace
