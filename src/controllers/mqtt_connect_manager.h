// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_MQTT_CONNECT_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_MQTT_CONNECT_MANAGER_H_

#include <QObject>

#include "mqtt/conn_info.h"
#include "mqtt/mqtt_client.h"

namespace hebo {

class MqttConnectManager : public QObject {
  Q_OBJECT
 public:
  explicit MqttConnectManager(QObject* parent = nullptr);

 public slots:
  void setConnectName(const QString& name) { conn_info_.name = name; };
  void setConnectClientId(const QString& client_id) { conn_info_.client_id = client_id; }
  void setConnectProtocol(const QString& protocol) { conn_info_.protocol = protocol; }
  void setConnectHost(const QString& host) { conn_info_.host = host; }
  void setConnectPort(int port) { conn_info_.port = port; }
  void setConnectQoS(int qos) { conn_info_.qos = static_cast<QoS>(qos); }
  void setConnectCleanSession(bool clean) { conn_info_.clean_session = clean; }

  void requestConnect();

 private:
  ConnInfo conn_info_{};
  QVector<MqttClient*> clients_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_MQTT_CONNECT_MANAGER_H_
