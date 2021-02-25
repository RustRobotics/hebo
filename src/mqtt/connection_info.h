// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
#define HEBOUI_SRC_MQTT_CONNECTION_INFO_H_

#include <QDebug>
#include <QObject>

namespace hebo {

struct ConnectionInfo {
  Q_GADGET
  Q_PROPERTY(QString name MEMBER name);
  Q_PROPERTY(QString clientId MEMBER client_id);
  Q_PROPERTY(QString protocol MEMBER protocol);
  Q_PROPERTY(QString host MEMBER host);
  Q_PROPERTY(int port MEMBER port);
  Q_PROPERTY(QoS qos MEMBER qos);
  Q_PROPERTY(QString username MEMBER username);
  Q_PROPERTY(QString password MEMBER password);
  Q_PROPERTY(bool tls MEMBER with_tls);
  Q_PROPERTY(bool cleanSession MEMBER clean_session);
  Q_PROPERTY(QString description MEMBER description);
  Q_PROPERTY(ConnectionState state MEMBER state);

 public:
  enum QoS : int32_t {
    kAtMostOnce = 0,
    kAtLeaseOnce = 1,
    kExactOnce = 2,
  };
  Q_ENUM(QoS);

  enum ConnectionState : int32_t {
    kDisconnected = 0,
    kConnecting = 1,
    kConnected = 2,
    kConnectFailed = 3,
    kDisconnecting = 4,
  };
  Q_ENUM(ConnectionState);

 public:
  QString name{};
  QString client_id{};
  QString protocol{};
  QString host{};
  int port{};
  QoS qos{QoS::kAtMostOnce};
  QString username{};
  QString password{};
  bool with_tls{false};
  bool clean_session{true};

  QString description{};
  ConnectionState state{ConnectionState::kDisconnected};
};

using QoS = ConnectionInfo::QoS;
using ConnectionState = ConnectionInfo::ConnectionState;

using ConnectionInfoList = QVector<ConnectionInfo>;

QString generateConnDescription(const ConnectionInfo& info);

QDebug operator<<(QDebug stream, const ConnectionInfo& info);

bool parseConnectionInfos(const QString& file, ConnectionInfoList& list);

QJsonObject dumpConnectionInfo(const ConnectionInfo& info);

bool dumpConnectionInfos(const QString& file, const ConnectionInfoList& list);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
