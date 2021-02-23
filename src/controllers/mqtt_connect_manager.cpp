// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/mqtt_connect_manager.h"

#include <QDebug>

namespace hebo {

MqttConnectManager::MqttConnectManager(QObject* parent) : QObject(parent) {
}

void MqttConnectManager::connect() {
  qDebug() << __func__;
}

}  // namespace hebo