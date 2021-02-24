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
  // Connections management
  // Protocol V3.1.1
  void addConnection(const QString& name,
                     const QString& client_id,
                     const QString& protocol,
                     const QString& host,
                     int port,
                     int qos,
                     bool clean_session) {
    ConnInfo conn_info{};
    conn_info.name = name;
    conn_info.client_id = client_id;
    conn_info.protocol = protocol;
    conn_info.host = host;
    conn_info.port = port;
    conn_info.qos = static_cast<QoS>(qos);
    conn_info.clean_session = clean_session;
    this->addConnInfo(conn_info);
  }

  const ConnInfoList& listConnections() const { return this->conn_info_list_; }

  void deleteConnection(const QString& name);

  void requestConnection(const QString& name);

 private:
  void addConnInfo(const ConnInfo& info);

  ConnInfoList conn_info_list_{};
  QString conn_file_;

  QVector<MqttClient*> clients_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_MQTT_CONNECT_MANAGER_H_
