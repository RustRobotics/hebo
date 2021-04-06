// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/models/protocol_model.h"

namespace hebo {

ProtocolModel::ProtocolModel(QObject* parent) : QAbstractListModel(parent) {

}

int ProtocolModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return 4;
}

QVariant ProtocolModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const int row = index.row();
  const auto protocol = static_cast<Protocol>(row);
  switch (role) {
    case Qt::DisplayRole: {
      return getProtocolName(protocol);
    }

    case RoleList::kIdRole: {
      return static_cast<int>(protocol);
    }

    default: {
      return {};
    }
  }
}

}  // namespace hebo