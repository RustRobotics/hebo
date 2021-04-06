// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_MQTT_CONNECT_CONFIG_H_
#define HEBO_SRC_MQTT_CONNECT_CONFIG_H_

#include <QDebug>
#include <QJsonObject>
#include <QString>

namespace hebo {

enum class ConnectionState : uint8_t {
  ConnectionDisconnected = 0,
  ConnectionConnecting = 1,
  ConnectionConnected = 2,
  ConnectionConnectFailed = 3,
  ConnectionDisconnecting = 4,
};

const char* dumpConnectionState(ConnectionState state);
QDebug operator<<(QDebug stream, ConnectionState state);

enum class QoS : uint8_t {
  AtMostOnce = 0,
  AtLeastOnce = 1,
  ExactOnce = 2,
};

const char* dumpQoS(QoS qos);
QDebug operator<<(QDebug stream, QoS qos);

struct ConnectConfig {
  QString id{};
  QString name{};
  QString client_id{};
  QString protocol{};
  QString host{};
  int port{};
  QoS qos{QoS::AtMostOnce};
  QString username{};
  QString password{};
  bool with_tls{false};

  // Advanced
  int timeout{10};
  int keep_alive{60};
  bool clean_session{true};
  bool auto_reconnect{false};

  // Last Will
  QString last_will_topic{};
  QoS last_will_qos{QoS::AtMostOnce};
  bool last_will_retain{};
  QByteArray last_will_payload{};

  QString description{};
};

using ConnectConfigList = QVector<ConnectConfig>;

QString generateConnDescription(const ConnectConfig& info);

QDebug operator<<(QDebug stream, const ConnectConfig& info);

bool parseConnectConfigs(const QString& file, ConnectConfigList& list);

QJsonObject dumpConnectConfig(const ConnectConfig& info);

bool dumpConnectConfigs(const QString& file, const ConnectConfigList& list);

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::ConnectionState);
Q_DECLARE_METATYPE(hebo::QoS);

#endif  // HEBO_SRC_MQTT_CONNECT_CONFIG_H_
