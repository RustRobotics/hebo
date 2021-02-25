// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_model.h"

#include <QDebug>
#include <QJsonObject>

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

}  // namespace

ConnectionModel::ConnectionModel(QObject* parent) : QAbstractListModel(parent) {

}

int ConnectionModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->list_.length();
}

QVariant ConnectionModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const ConnectionInfo& info = this->list_.at(index.row());
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
    default: {
      qWarning() << "Invalid role:" << role;
      return {};
    }
  }
}

QHash<int, QByteArray> ConnectionModel::roleNames() const {
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
  };
}

void ConnectionModel::addConnectionInfo(const ConnectionInfo& info) {
  const int len = this->list_.length();
  this->beginInsertRows(QModelIndex(), len, len + 1);
  this->list_.append(info);
  this->endInsertRows();
}

void ConnectionModel::setList(const ConnectionInfoList& list) {
  this->beginResetModel();
  this->list_ = list;
  this->endResetModel();
}

bool ConnectionModel::getConnectionInfo(const QString& name, ConnectionInfo& info) const {
  for (const auto& item : this->list_) {
    if (item.name == name) {
      info = item;
      return true;
    }
  }
  return false;
}

QVariantMap ConnectionModel::row(int row) const {
  if (row >= 0 && row < this->list_.length()) {
    const QJsonObject object = dumpConnectionInfo(this->list_.at(row));
    return object.toVariantMap();
  } else {
    return {};
  }
}

bool ConnectionModel::deleteConnectionInfo(const QString& name) {
  int index;
  for (index = 0; index < this->list_.length(); ++index) {
    if (this->list_.at(index).name == name) {
      break;
    }
  }
  if (index < this->list_.length()) {
    this->list_.removeAt(index);
  }

  return index < this->list_.length();
}

}  // namespace hebo
