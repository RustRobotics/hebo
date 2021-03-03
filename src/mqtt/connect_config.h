// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECT_CONFIG_H_
#define HEBOUI_SRC_MQTT_CONNECT_CONFIG_H_

#include <QDebug>
#include <QJsonObject>
#include <QString>

namespace hebo {

class HeboEnums : public QObject {
  Q_OBJECT
 public:
  explicit HeboEnums(QObject* parent = nullptr);

  enum ConnectionState : int32_t {
    ConnectionDisconnected = 0,
    ConnectionConnecting = 1,
    ConnectionConnected = 2,
    ConnectionConnectFailed = 3,
    ConnectionDisconnecting = 4,
  };
  Q_ENUM(ConnectionState);


  enum QoS : int32_t {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactOnce = 2,
  };
  Q_ENUM(QoS);
};

using QoS = HeboEnums::QoS;
using ConnectionState = HeboEnums::ConnectionState;

struct ConnectConfig {
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
};

using ConnectConfigList = QVector<ConnectConfig>;

QString generateConnDescription(const ConnectConfig& info);

QDebug operator<<(QDebug stream, const ConnectConfig& info);

bool parseConnectConfigs(const QString& file, ConnectConfigList& list);

QJsonObject dumpConnectConfig(const ConnectConfig& info);

bool dumpConnectConfigs(const QString& file, const ConnectConfigList& list);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECT_CONFIG_H_
