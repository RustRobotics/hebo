// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/mqtt_client.h"

namespace hebo {

MqttClient::MqttClient(QObject* parent)
    : QObject(parent),
      worker_thread_(new QThread(this)),
      subscriptions_(new SubscriptionModel(this)),
      messages_(new MessageStreamModel(this)),
      internal_(new InternalClient()) {
  qRegisterMetaType<ConnectionState>("ConnectionState");
  qRegisterMetaType<ConnectionState>("HeboEnums.ConnectionState");
  qRegisterMetaType<ConnectConfig>("ConnectConfig");
  qRegisterMetaType<QoS>("QoS");

  this->internal_->moveToThread(this->worker_thread_);
  this->initSignals();

  this->worker_thread_->start();
}

MqttClient::~MqttClient() {
  this->worker_thread_->quit();
  this->worker_thread_->wait();
}

void MqttClient::initSignals() {
  connect(this->worker_thread_, &QThread::finished,
          this->internal_, &InternalClient::deleteLater);
  connect(this->worker_thread_, &QThread::finished,
          this->worker_thread_, &QThread::deleteLater);

  connect(this->internal_, &InternalClient::stateChanged,
          this, &MqttClient::setState);

  connect(this->internal_, &InternalClient::messageReceived,
          this->messages_, &MessageStreamModel::addMessage);

  connect(this->subscriptions_, &SubscriptionModel::dataChanged, [=]() {
    emit this->subscriptionsChanged(this->subscriptions_);
  });

  connect(this->messages_, &MessageStreamModel::dataChanged, [=]() {
    emit this->messagesChanged(this->messages_);
  });
}

void MqttClient::setState(ConnectionState state) {
  qDebug() << __func__ << state;
  this->state_ = state;
  emit this->stateChanged(state);
}

void MqttClient::requestConnect() {
  emit this->internal_->requestConnect(this->config_);
}

void MqttClient::requestDisconnect() {
  Q_ASSERT(this->state_ == ConnectionState::ConnectionConnected);
  this->setState(ConnectionState::ConnectionDisconnecting);
  emit this->internal_->requestDisconnect();
}

void MqttClient::requestSubscribe(const QString& topic, int qos, const QString& color) {
  qDebug() << __func__ << topic;

  Q_ASSERT(this->state_ == ConnectionState::ConnectionConnected);
  if (this->state_ != ConnectionState::ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }

  if (this->subscriptions_->hasSubscription(topic)) {
    qWarning() << "Topic already subscribed:" << topic;
    return;
  }

  this->subscriptions_->addSubscription(topic, qos, color);
  emit this->internal_->requestSubscribe(topic, static_cast<QoS>(qos));
}

void MqttClient::requestUnsubscribe(const QString& topic) {
  Q_ASSERT(this->state_ == ConnectionState::ConnectionConnected);
  if (this->state_ != ConnectionState::ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }

  if (this->subscriptions_->removeSubscription(topic)) {
    emit this->internal_->requestUnsubscribe(topic);
  } else {
    qWarning() << "Topic with name not subscribed:" << topic;
  }
}

void MqttClient::requestPublish(const QString& topic, int qos, const QByteArray& payload) {
  Q_ASSERT(this->state_ == ConnectionState::ConnectionConnected);
  if (this->state_ != ConnectionState::ConnectionConnected) {
    qWarning() << "Invalid state:" << this->state_;
    return;
  }
  emit this->internal_->requestPublish(topic, static_cast<QoS>(qos), payload);
  MqttMessage message{};
  message.topic = topic;
  message.qos = static_cast<QoS>(qos);
  message.is_publish = true;
  message.payload = payload;
  this->messages_->addMessage(message);
}

}  // namespace
