// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_state.h"

namespace hebo {

TestClass::TestClass(QObject* parent) : QAbstractListModel(parent) {
  this->setObjectName("TestClass");

}

int TestClass::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return 0;
}

QVariant TestClass::data(const QModelIndex& index, int role) const {
  Q_UNUSED(index);
  Q_UNUSED(role);
  return QVariant();
}

QHash<int, QByteArray> TestClass::roleNames() const {
  return QAbstractItemModel::roleNames();
}

}  // namespace hebo
