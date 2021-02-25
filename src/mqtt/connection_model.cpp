// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_model.h"

#include <QDebug>

namespace hebo {

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
    case kConnectionNameRole: {
      return info.name;
    }
    case kConnectionClientIdRole: {
      return info.client_id;
    }
    case kConnectionProtocolRole: {
      return info.protocol;
    }
    case kConnectionHostRole: {
      return info.host;
    }
    case kConnectionPortRole: {
      return info.port;
    }
    case kConnectionQoSRole: {
      return static_cast<int>(info.qos);
    }
    case kConnectionUsernameRole: {
      return info.username;
    }
    case kConnectionPasswordRole: {
      return info.password;
    }
    case kConnectionTlsRole: {
      return info.with_tls;
    }
    case kConnectionCleanSessionRole: {
      return info.clean_session;
    }
    case kConnectionDescriptionRole: {
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
      {kConnectionNameRole, "name"},
      {kConnectionClientIdRole, "clientId"},
      {kConnectionProtocolRole, "protocol"},
      {kConnectionHostRole, "host"},
      {kConnectionPortRole, "port"},
      {kConnectionQoSRole, "qos"},
      {kConnectionUsernameRole, "username"},
      {kConnectionPasswordRole, "password"},
      {kConnectionTlsRole, "tls"},
      {kConnectionCleanSessionRole, "cleanSession"},
      {kConnectionDescriptionRole, "description"},
  };
}

void ConnectionModel::addConnectionInfo(const ConnectionInfo& info) {
  this->beginResetModel();
  this->list_.append(info);
  this->endResetModel();
}

}  // namespace hebo