// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_MQTT_MQTT_CLIENT_H_
#define HEBO_SRC_MQTT_MQTT_CLIENT_H_

#include <QColor>
#include <QObject>
#include <QThread>

#include "formats/connect_config.h"
#include "mqtt/internal_client.h"
#include "mqtt/message_stream_model.h"
#include "mqtt/subscription_model.h"

namespace hebo {

class MqttClient : public QObject {
  Q_OBJECT
  Q_PROPERTY(ConnectionState state READ state NOTIFY stateChanged);
  // TODO(Shaohua): Remove these properties.
  Q_PROPERTY(SubscriptionModel* subscriptions READ subscriptions NOTIFY subscriptionsChanged);
  Q_PROPERTY(MessageStreamModel* messages READ messages NOTIFY messagesChanged);

 public:
  explicit MqttClient(QObject* parent = nullptr);
  ~MqttClient() override;

  [[nodiscard]] const ConnectConfig& config() const { return this->config_; }

  void setConfig(const ConnectConfig& config) { this->config_ = config; }

  [[nodiscard]] ConnectionState state() const { return this->state_; }

  [[nodiscard]] SubscriptionModel* subscriptions() const { return this->subscriptions_; }

  [[nodiscard]] MessageStreamModel* messages() const { return this->messages_; }

 public slots:
  void requestConnect();
  void requestDisconnect();
  void requestSubscribe(const QString& topic, QoS qos, const QColor& color);
  void requestUnsubscribe(const QString& topic);
  void requestPublish(const QString& topic, const QByteArray& payload, QoS qos, bool retain);

 signals:
  // TODO(Shaohua): Remove response signals.
  void disconnectResult(bool ok, const QString& error);
  void subscribeResult(const QString& topic, bool ok, const QString& error);
  void unsubscribeResult(const QString& topic, bool ok, const QString& error);
  void publishResult(const QString& topic, bool ok, const QString& error);

  void stateChanged(ConnectionState state);
  void subscriptionsChanged(SubscriptionModel* model);
  void messagesChanged(MessageStreamModel* model);

 private:
  void initSignals();

  void setState(ConnectionState state);

  QThread* worker_thread_;
  ConnectConfig config_{};
  ConnectionState state_{ConnectionState::ConnectionDisconnected};
  SubscriptionModel* subscriptions_;
  MessageStreamModel* messages_;

  InternalClient* internal_;
};

}  // namespace hebo

#endif  // HEBO_SRC_MQTT_MQTT_CLIENT_H_
