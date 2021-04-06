// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/models/qos_model.h"

namespace hebo {

QoSModel::QoSModel(QObject* parent) : QAbstractListModel(parent) {

}

int QoSModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return 3;
}

QVariant QoSModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const int row = index.row();
  const auto qos = static_cast<QoS>(row);
  switch (role) {
    case Qt::DisplayRole: {
      return dumpQoS(qos);
    }
    case kIdRole: {
      return static_cast<int>(qos);
    }
    default: {
      return {};
    }
  }
}

}  // namespace hebo