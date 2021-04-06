// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/models/version_model.h"

namespace hebo {

VersionModel::VersionModel(QObject* parent) : QAbstractListModel(parent) {

}

int VersionModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return 2;
}

QVariant VersionModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const int row = index.row();
  const auto version = static_cast<MqttVersion>(row);
  switch (role) {
    case Qt::DisplayRole: {
      return getMqttVersionName(version);
    }
    case kIdRole: {
      return static_cast<int>(version);
    }
    default: {
      return {};
    }
  }
}

}  // namespace hebo