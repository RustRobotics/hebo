// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_model.h"

namespace hebo {

ConnectionModel::ConnectionModel(QObject* parent) : QAbstractItemModel(parent) {

}

int ConnectionModel::rowCount(const QModelIndex& parent) const {
  return 0;
}

QVariant ConnectionModel::data(const QModelIndex& index, int role) const {
  return QVariant();
}

QHash<int, QByteArray> ConnectionModel::roleNames() const {
  return QAbstractItemModel::roleNames();
}

}  // namespace hebo