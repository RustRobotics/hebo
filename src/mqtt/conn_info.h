// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONN_INFO_H_
#define HEBOUI_SRC_MQTT_CONN_INFO_H_

#include <QDebug>
#include <QMetaType>
#include <QString>
#include <QObject>

namespace hebo {

enum class QoS : uint8_t {
  kAtMostOnce,
  kAtLeaseOnce,
  kExactOnce,
};

class ConnInfo : public QObject {
  Q_OBJECT
  Q_PROPERTY(QString name READ name WRITE setName)

 public:
  explicit ConnInfo(QObject* parent = nullptr);

  const QString& name() const { return this->name_; }

 public slots:
  void setName(const QString& name) {
    this->name_ = name;
  };

 private:
  QString name_{};
//  QString client_id{};
//  QString protocol{};
//  QString host{};
//  uint16_t port{};
//  QoS qos{QoS::kAtMostOnce};
//  QString username{};
//  QString password{};
//  bool with_tls{false};
//  bool clean_session;
};

QDebug operator<<(QDebug stream, const ConnInfo& info);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONN_INFO_H_
