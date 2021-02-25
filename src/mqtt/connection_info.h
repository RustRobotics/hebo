// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
#define HEBOUI_SRC_MQTT_CONNECTION_INFO_H_

#include <QDebug>
#include <QObject>

namespace hebo {
Q_NAMESPACE

enum QoS : int32_t {
  AtMostOnce = 0,
  AtLeaseOnce = 1,
  ExactOnce = 2,
};
Q_ENUM_NS(QoS);

enum ConnectionState : int32_t {
  ConnectionDisconnected = 0,
  ConnectionConnecting = 1,
  ConnectionConnected = 2,
  ConnectionConnectFailed = 3,
  ConnectionDisconnecting = 4,
};
Q_ENUM_NS(ConnectionState);

struct ConnectionInfo{
  QString name{};
  QString client_id{};
  QString protocol{};
  QString host{};
  int port{};
  QoS qos{QoS::AtMostOnce};
  QString username{};
  QString password{};
  bool with_tls{false};
  bool clean_session{true};

  QString description{};
  ConnectionState state{ConnectionState::ConnectionDisconnected};
};

using ConnectionInfoList = QVector<ConnectionInfo>;

QString generateConnDescription(const ConnectionInfo& info);

QDebug operator<<(QDebug stream, const ConnectionInfo& info);

bool parseConnectionInfos(const QString& file, ConnectionInfoList& list);

QJsonObject dumpConnectionInfo(const ConnectionInfo& info);

bool dumpConnectionInfos(const QString& file, const ConnectionInfoList& list);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
