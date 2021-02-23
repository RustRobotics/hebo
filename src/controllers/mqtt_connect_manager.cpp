// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/mqtt_connect_manager.h"

#include <QDebug>

namespace hebo {

MqttConnectManager::MqttConnectManager(QObject* parent) : QObject(parent) {
}

void MqttConnectManager::requestConnect() {
  qDebug() << __func__ << conn_info_;
  auto* client = new MqttClient();
  client->requestConnect(conn_info_);
  this->clients_.append(client);
}

}  // namespace hebo