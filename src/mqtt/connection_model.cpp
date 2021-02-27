// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_model.h"

#include <QDebug>
#include <QJsonObject>

namespace hebo {


ConnectionModel::ConnectionModel(QObject* parent) : QAbstractListModel(parent) {
  qRegisterMetaType<ConnectionInfo>("ConnectionInfo");
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

void ConnectionModel::updateConnectionState(const QString& name, ConnectionState state) {
  qDebug() << __func__ << name << state;
  this->beginResetModel();
  for (auto& item : this->list_) {
    if (item.name == name) {
      item.state = state;
      break;
    }
  }
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
