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
      conn_file_(getJsonFile()),
      model_(new ConnectionModel(this)) {

  // Load connections on startup.
  this->loadConnInfo();
}

void ConnectManager::deleteConnection(const QString& name) {
  if (!this->model_->deleteConnectionInfo(name)) {
    qWarning() << "Invalid connection info:" << name;
  } else {
    this->saveConnInfo();
  }

  if (this->clients_.contains(name)) {
    emit this->clients_.take(name)->requestDisconnect();
  }
}

void ConnectManager::requestConnect(const QString& name) {
  ConnectionInfo info;
  if (!this->model_->getConnectionInfo(name, info)) {
    qWarning() << "Invalid connection info:" << name;
    return;
  }

  if (!this->clients_.contains(name)) {

  } else {
    const auto client = this->clients_[name];
    Q_ASSERT(!client.isNull());
    client->requestConnect(info);
  }
}

void ConnectManager::addConnection(const QString& name,
                   const QString& client_id,
                   const QString& protocol,
                   const QString& host,
                   int port,
                   int qos,
                   bool clean_session) {
  ConnectionInfo conn_info{};
  conn_info.name = name;
  conn_info.client_id = client_id;
  conn_info.protocol = protocol;
  conn_info.host = host;
  conn_info.port = port;
  conn_info.qos = static_cast<QoS>(qos);
  conn_info.clean_session = clean_session;
  conn_info.description = generateConnDescription(conn_info);

  this->model_->addConnectionInfo(conn_info);

  // save to local file
  this->saveConnInfo();
}


void ConnectManager::saveConnInfo() {
  if (!dumpConnectionInfos(this->conn_file_, this->model_->list())) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
}

void ConnectManager::loadConnInfo() {
  ConnectionInfoList list{};
  const bool ok = parseConnectionInfos(this->conn_file_, list);
  if (!ok) {
    qWarning() << "Failed to parse conn info file:" << this->conn_file_;
    return;
  }

  this->model_->setList(list);
  // TODO(Shaohua): Create mqtt client map.
}

}  // namespace hebo
