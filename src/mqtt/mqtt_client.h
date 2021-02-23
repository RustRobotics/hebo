// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
#define HEBOUI_SRC_MQTT_MQTT_CLIENT_DEMO_H_

#include <QObject>

namespace hebo {

class MqttClient : public QObject {
  Q_OBJECT
 public:
  explicit MqttClient(QObject* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_MQTT_CLIENT_H_
