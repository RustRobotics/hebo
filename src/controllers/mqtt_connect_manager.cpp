// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/mqtt_connect_manager.h"

#include <QDebug>
#include <QDir>
#include <QStandardPaths>

namespace hebo {
namespace {

QString getJsonFile() {
  const QStringList dirs = QStandardPaths::standardLocations(QStandardPaths::AppConfigLocation);
  Q_ASSERT(!dirs.isEmpty());
  QDir dir(dirs.first());
  dir.cdUp();
  return dir.absoluteFilePath("connections.json");
}

}  // namespace

MqttConnectManager::MqttConnectManager(QObject* parent)
    : QObject(parent),
      conn_file_(getJsonFile()) {
  qDebug() << "conn file:" << conn_file_;
}

void MqttConnectManager::deleteConnection(const QString& name) {
  int index;
  for (index = 0; index < conn_info_list_.length(); ++index) {
    if (conn_info_list_.at(index).name == name) {
      break;
    }
  }
  if (index == conn_info_list_.length()) {
    qWarning() << "Failed to find ConnInfo with name:" << name;
    return;
  }

  // disconnect

  // delete from list
  conn_info_list_.removeAt(index);

  // Save to file
  if (!dumpConnInfos(conn_file_, conn_info_list_)) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
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
  if (!dumpConnInfos(conn_file_, conn_info_list_)) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
}

}  // namespace hebo