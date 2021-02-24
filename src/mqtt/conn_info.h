// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONN_INFO_H_
#define HEBOUI_SRC_MQTT_CONN_INFO_H_

#include <QDebug>
#include <QMetaType>
#include <QString>

namespace hebo {

constexpr const int kAtMostOnce = 0;
constexpr const int kAtLeaseOnce = 1;
constexpr const int kExactOnce = 2;

struct ConnInfo {
 private:
  Q_GADGET
  Q_PROPERTY(QString name MEMBER name);
  Q_PROPERTY(QString clientId MEMBER client_id);
  Q_PROPERTY(QString protocol MEMBER protocol);
  Q_PROPERTY(QString host MEMBER host);
  Q_PROPERTY(int port MEMBER port);
  Q_PROPERTY(int qos MEMBER qos);
  Q_PROPERTY(QString username MEMBER username);
  Q_PROPERTY(QString password MEMBER password);
  Q_PROPERTY(bool tls MEMBER with_tls);
  Q_PROPERTY(bool cleanSession MEMBER clean_session);

 public:
  QString name{};
  QString client_id{};
  QString protocol{};
  QString host{};
  uint16_t port{};
  int qos{kAtMostOnce};
  QString username{};
  QString password{};
  bool with_tls{false};
  bool clean_session;
};

using ConnInfoList = QVector<ConnInfo>;

QDebug operator<<(QDebug stream, const ConnInfo& info);

bool parseConnInfos(const QString& file, ConnInfoList& list);

bool dumpConnInfos(const QString& file, const ConnInfoList& list);

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::ConnInfo);

#endif  // HEBOUI_SRC_MQTT_CONN_INFO_H_
