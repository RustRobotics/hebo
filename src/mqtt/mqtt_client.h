// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
#define HEBOUI_SRC_MQTT_MQTT_CLIENT_H_

#include <QObject>

#include "mqtt/conn_info.h"

namespace hebo {

struct MqttClientPrivate;

class MqttClient : public QObject {
  Q_OBJECT
 public:
  explicit MqttClient(QObject* parent = nullptr);
  ~MqttClient() override;

 public slots:

 signals:
  void requestConnect(const ConnInfo& info);
  void connectResult(bool ok, const QString& error);

  void requestDisconnect();

 private slots:
  void doConnect(const ConnInfo& info);

  void doDisconnect();

 private:
  void initSignals();

  void initClient();

  MqttClientPrivate* p_;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
