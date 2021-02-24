// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/connect_manager.h"

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

ConnectManager::ConnectManager(QObject* parent)
    : QObject(parent),
      conn_file_(getJsonFile()) {
  qDebug() << "conn file:" << conn_file_;
}

void ConnectManager::deleteConnection(const QString& name) {
  int index;
  for (index = 0; index < conn_list_.length(); ++index) {
    if (conn_list_.at(index).info.name == name) {
      break;
    }
  }
  if (index == conn_list_.length()) {
    qWarning() << "Failed to find ConnInfo with name:" << name;
    return;
  }

  auto& item = conn_list_[index];
  // disconnect
  if (item.client.isNull()) {
    emit item.client->requestDisconnect();
    item.client.clear();
  }

  // delete from list
  conn_list_.removeAt(index);

  // Save to file
  this->saveConnInfo();
}

void ConnectManager::requestConnect(const QString& name) {
  for (auto& item : conn_list_) {
    if (item.info.name == name) {
      if (item.client.isNull()) {
        item.client.reset(new MqttClient());
        item.client->requestConnect(item.info);
      } else {
        qWarning() << "MqttClient is not null:" << name;
      }
      return;
    }
  }
  qWarning() << "Failed to find ConnInfo with name:" << name;
}

void ConnectManager::addConnInfo(const ConnInfo& info) {
  ConnectStateInfo item{};
  item.info = info;
  this->conn_list_.append(item);

  // save to local file
  this->saveConnInfo();
}

void ConnectManager::saveConnInfo() {
  ConnInfoList info_list{};
  for (const auto& item : conn_list_) {
    info_list.append(item.info);
  }

  if (!dumpConnInfos(conn_file_, info_list)) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
}

}  // namespace hebo