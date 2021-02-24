// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/mqtt_connect_manager.h"

#include <QDebug>

namespace hebo {

MqttConnectManager::MqttConnectManager(QObject* parent) : QObject(parent) {
}


void MqttConnectManager::deleteConnection(const QString& name) {
  for (const auto& info : conn_info_list_) {
    if (info.name == name) {
      // disconnect
      // delete from list
      // Save to file
      return;
    }
  }
  qWarning() << "Failed to find ConnInfo with name:" << name;
}

void MqttConnectManager::requestConnection(const QString& name) {
  for (const auto& info : conn_info_list_) {
    if (info.name == name) {
      auto* client = new MqttClient();
      client->requestConnect(info);
      this->clients_.append(client);
      return;
    }
  }
  qWarning() << "Failed to find ConnInfo with name:" << name;

}

void MqttConnectManager::addConnInfo(const ConnInfo& info) {
  this->conn_info_list_.append(info);
  // save to local file
}

}  // namespace hebo