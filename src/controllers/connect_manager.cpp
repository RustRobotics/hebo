// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/connect_manager.h"

#include <QDebug>
#include <QDir>
#include <QStandardPaths>

#include "base/random.h"

namespace hebo {
namespace {

constexpr const char* kName = "name";
constexpr const char* kClientId = "clientId";
constexpr const char* kProtocol = "protocol";
constexpr const char* kHost = "host";
constexpr const char* kPort = "port";
constexpr const char* kQoS = "qos";
constexpr const char* kUsername = "username";
constexpr const char* kPassword = "password";
constexpr const char* kTls = "tls";
constexpr const char* kCleanSession = "cleanSession";
constexpr const char* kDescription = "description";
constexpr const char* kConnectionState = "connectionState";

QString getJsonFile() {
  const QStringList dirs = QStandardPaths::standardLocations(QStandardPaths::AppConfigLocation);
  Q_ASSERT(!dirs.isEmpty());
  QDir dir(dirs.first());
  dir.cdUp();
  return dir.absoluteFilePath("connections.json");
}

}  // namespace

ConnectManager::ConnectManager(QObject* parent)
    : QAbstractListModel(parent),
      conn_file_(getJsonFile()) {
  // Load connections on startup.
  this->loadConnInfo();
}


int ConnectManager::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->configs_.length();
}

QVariant ConnectManager::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const ConnectConfig& info = this->configs_.at(index.row());
  switch (role) {
    case kNameRole: {
      return info.name;
    }
    case kClientIdRole: {
      return info.client_id;
    }
    case kProtocolRole: {
      return info.protocol;
    }
    case kHostRole: {
      return info.host;
    }
    case kPortRole: {
      return info.port;
    }
    case kQoSRole: {
      return static_cast<int>(info.qos);
    }
    case kUsernameRole: {
      return info.username;
    }
    case kPasswordRole: {
      return info.password;
    }
    case kTlsRole: {
      return info.with_tls;
    }
    case kCleanSessionRole: {
      return info.clean_session;
    }
    case kDescriptionRole: {
      return info.description;
    }
    case kConnectionStateRole: {
      if (this->clients_.contains(info.name)) {
        auto* client = this->clients_.value(info.name);
        Q_ASSERT(client != nullptr);
        qDebug() << "client state:" << client->state();
        return client->state();
      } else {
        return ConnectionState::ConnectionDisconnected;
      }
    }
    default: {
      qWarning() << "Invalid role:" << role;
      return {};
    }
  }
}

QHash<int, QByteArray> ConnectManager::roleNames() const {
  // Map role index to qml property name.
  return {
      {kNameRole, kName},
      {kClientIdRole, kClientId},
      {kProtocolRole, kProtocol},
      {kHostRole, kHost},
      {kPortRole, kPort},
      {kQoSRole, kQoS},
      {kUsernameRole, kUsername},
      {kPasswordRole, kPassword},
      {kTlsRole, kTls},
      {kCleanSessionRole, kCleanSession},
      {kDescriptionRole, kDescription},
      {kConnectionStateRole, kConnectionState},
  };
}

void ConnectManager::addConnection(const QString& name,
                                   const QString& client_id,
                                   const QString& protocol,
                                   const QString& host,
                                   int port,
                                   QoS qos,
                                   bool clean_session) {
  ConnectConfig config{};
  config.name = name;
  config.client_id = client_id;
  config.protocol = protocol;
  config.host = host;
  config.port = port;
  config.qos = qos;
  config.clean_session = clean_session;
  config.description = generateConnDescription(config);

  this->beginResetModel();
  this->configs_.append(config);
  this->endResetModel();

  // save to local file
  this->saveConnInfo();
}


void ConnectManager::saveConnInfo() {
  if (!dumpConnectConfigs(this->conn_file_, this->configs_)) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
}

void ConnectManager::loadConnInfo() {
  const bool ok = parseConnectConfigs(this->conn_file_, this->configs_);
  if (!ok) {
    qWarning() << "Failed to parse conn info file:" << this->conn_file_;
    return;
  }
}

QVariantMap ConnectManager::row(int index) {
  Q_ASSERT(index >= 0 && index < this->configs_.length());
  return dumpConnectConfig(this->configs_.at(index)).toVariantMap();
}

QVariantMap ConnectManager::rowByName(const QString& name) {
  for (const auto& config : this->configs_) {
    if (config.name == name) {
      return dumpConnectConfig(config).toVariantMap();
    }
  }
  qWarning() << "Failed to find config with name:" << name;
  return {};
}

MqttClient* ConnectManager::client(const QString& name) {
  if (this->clients_.contains(name)) {
    auto* client = this->clients_.value(name);
    Q_ASSERT(client != nullptr);
    return client;
  }

  for (const auto& config : this->configs_) {
    if (config.name == name) {
      auto* new_client = new MqttClient(this);
      connect(new_client, &MqttClient::stateChanged, [=]() {
        for (int index = 0; index < this->configs_.length(); ++index) {
          if (this->configs_.at(index).name == name) {
            emit this->dataChanged(this->index(index), this->index(index));
            return;
          }
        }
        qWarning() << "Failed to find config with name:" << name;
      });

      new_client->setConfig(config);
      this->clients_.insert(name, new_client);
      qDebug() << "Create new client:" << new_client;
      return new_client;
    }
  }

  qWarning() << "Invalid connection name:" << name;
  return nullptr;
}

QString ConnectManager::newClientId() const {
  return "hebo_" + randomClientId();
}

}  // namespace hebo