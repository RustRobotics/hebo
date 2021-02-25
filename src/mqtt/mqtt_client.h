// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
#define HEBOUI_SRC_MQTT_MQTT_CLIENT_H_

#include <QObject>

#include "mqtt/connection_info.h"

namespace hebo {

struct MqttClientPrivate;

class MqttClient : public QObject {
  Q_OBJECT
 public:
  explicit MqttClient(QObject* parent = nullptr);
  ~MqttClient() override;

 public slots:

 signals:
  void requestConnect(const ConnectionInfo& info);
  void connectResult(bool ok, const QString& error);

  void requestDisconnect();
  void disconnectResult(bool ok, const QString& error);

  void requestSubscribe(const QString& topic, QoS qos);
  void subscribeResult(const QString& topic, bool ok, const QString& error);

  void requestUnsubscribe(const QString& topic);
  void unsubscribeResult(const QString& topic, bool ok, const QString& error);

  void requestPublish(const QString& topic, QoS qos, const QByteArray& payload);
  void publishResult(const QString& topic, bool ok, const QString& error);

  void connectionStateChanged(ConnectionState state);

 protected:
  void timerEvent(QTimerEvent* event) override;

 private slots:
  void doConnect(const ConnectionInfo& info);

  void doDisconnect();

  void doSubscribe(const QString& topic, QoS qos);

  void doUnsubscribe(const QString& topic);

  void doPublish(const QString& topic, QoS qos, const QByteArray& payload);

 private:
  void initSignals();

  void initClient();

  MqttClientPrivate* p_;
};

using MqttClientPtr = QSharedPointer<MqttClient>;

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
