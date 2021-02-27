// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/connect_manager.h"

#include <QDebug>
#include <QDir>
#include <QStandardPaths>

namespace hebo {

ConnectManager::ConnectManager(QObject* parent)
    : QObject(parent),
      model_(new ConnectionModel(this)) {



  connect(this->model_, &ConnectionModel::dataChanged, [=]() {
    emit this->modelChanged(this->model_);
  });
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
    QSharedPointer<MqttClient> client(new MqttClient());
    connect(client.data(), &MqttClient::connectionStateChanged, [=](ConnectionState state) {
      this->model_->updateConnectionState(name, state);
    });

    this->clients_.insert(name, client);
    client->requestConnect(info);
  } else {
    const auto client = this->clients_[name];
    Q_ASSERT(!client.isNull());
    client->requestConnect(info);
  }
}





}  // namespace hebo
