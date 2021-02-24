// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_

#include <QObject>
#include <QSharedPointer>

#include "mqtt/conn_info.h"
#include "mqtt/conn_state.h"
#include "mqtt/mqtt_client.h"

namespace hebo {

struct ConnectStateInfo {
 private:
  Q_GADGET
  Q_PROPERTY(ConnInfo info MEMBER info);
  Q_PROPERTY(ConnectState state MEMBER state);

 public:
  ConnInfo info{};
  ConnectState state{ConnectState::kDisconnected};
  QSharedPointer<MqttClient> client{nullptr};
};
using ConnectStateInfoList =  QVector<ConnectStateInfo>;

class ConnectManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(ConnectStateInfoList connList READ connList NOTIFY connListChanged)

 public:
  explicit ConnectManager(QObject* parent = nullptr);

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
    conn_info.qos = qos;
    conn_info.clean_session = clean_session;
    this->addConnInfo(conn_info);
  }

  void deleteConnection(const QString& name);

  void requestConnect(const QString& name);

  const ConnectStateInfoList& connList() const {
    return this->conn_list_;
  }

 signals:
  void connListChanged(const ConnectStateInfoList& list);

 private:
  void addConnInfo(const ConnInfo& info);

  void saveConnInfo();

  QString conn_file_;
  ConnectStateInfoList conn_list_{};
};

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::ConnectStateInfo);

#endif  // HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
