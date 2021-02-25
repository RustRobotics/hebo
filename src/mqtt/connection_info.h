// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
#define HEBOUI_SRC_MQTT_CONNECTION_INFO_H_

#include <QDebug>
#include <QMetaType>
#include <QString>

namespace hebo {

enum class QoS : uint8_t {
  kAtMostOnce = 0,
  kAtLeaseOnce = 1,
  kExactOnce = 2,
};

struct ConnectionInfo {
  QString name{};
  QString client_id{};
  QString protocol{};
  QString host{};
  uint16_t port{};
  QoS qos{QoS::kAtMostOnce};
  QString username{};
  QString password{};
  bool with_tls{false};
  bool clean_session{true};
  QString description{};
};

using ConnectionInfoList = QVector<ConnectionInfo>;

QString generateConnDescription(const ConnectionInfo& info);

QDebug operator<<(QDebug stream, const ConnectionInfo& info);

bool parseConnInfos(const QString& file, ConnInfoList& list);

bool dumpConnInfos(const QString& file, const ConnInfoList& list);

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::ConnectionInfo);

#endif  // HEBOUI_SRC_MQTT_CONNECTION_INFO_H_