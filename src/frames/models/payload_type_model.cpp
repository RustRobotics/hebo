// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/models/payload_type_model.h"

namespace hebo {

PayloadTypeModel::PayloadTypeModel(QObject* parent)
  : QAbstractListModel(parent),
    type_list_({"PlainText", "Base64", "JSON", "Hex"}) {
}

int PayloadTypeModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->type_list_.length();
}

QVariant PayloadTypeModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }
  const int row = index.row();
  switch (role) {
    case Qt::DisplayRole:  // fall through
    case kNameRole: {
      return this->type_list_.at(row);
    }
    case kIdRole: {
      return row;
    }
    default: {
      return {};
    }
  }
}

}  // namespace hebo