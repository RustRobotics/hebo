// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
#define HEBOUI_SRC_MQTT_MQTT_CLIENT_H_

#include <QObject>
#include <QThread>

#include "mqtt/connect_config.h"

namespace hebo {

struct MqttClientPrivate;

class MqttClient : public QObject {
  Q_OBJECT
  Q_PROPERTY(ConnectionState state READ state NOTIFY stateChanged);

 public:
  enum ConnectionState : int32_t {
    ConnectionDisconnected = 0,
    ConnectionConnecting = 1,
    ConnectionConnected = 2,
    ConnectionConnectFailed = 3,
    ConnectionDisconnecting = 4,
  };
  Q_ENUM(ConnectionState);

  explicit MqttClient(QObject* parent = nullptr);
  ~MqttClient() override;

  void setConfig(const ConnectConfig& config) { this->config_ = config; }

  [[nodiscard]] ConnectionState state() const { return this->state_; }

 public slots:
  void requestConnect();
  void requestDisconnect();
  void requestSubscribe(const QString& topic, QoS qos);
  void requestUnsubscribe(const QString& topic);
  void requestPublish(const QString& topic, int qos, const QByteArray& payload);

 signals:
  void connectResult(bool ok, const QString& error);
  void disconnectResult(bool ok, const QString& error);
  void subscribeResult(const QString& topic, bool ok, const QString& error);
  void unsubscribeResult(const QString& topic, bool ok, const QString& error);
  void publishResult(const QString& topic, bool ok, const QString& error);
  void stateChanged(ConnectionState state);

 protected:
  void timerEvent(QTimerEvent* event) override;

 private:
  void initSignals();

  void setState(ConnectionState state);

  QThread* worker_thread_;
  ConnectConfig config_{};
  ConnectionState state_{ConnectionState::ConnectionDisconnected};
  int timer_id_{-1};
  MqttClientPrivate* p_;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
