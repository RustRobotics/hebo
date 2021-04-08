// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/models/color_palette_model.h"

namespace hebo {

ColorPaletteModel::ColorPaletteModel(QObject* parent) : QAbstractListModel(parent) {

}

int ColorPaletteModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->palette_.length();
}

QVariant ColorPaletteModel::data(const QModelIndex& index, int role) const {
  if (index.isValid() && role == Qt::DecorationRole) {
    return this->palette_.at(index.row());
  }

  return {};
}

void ColorPaletteModel::setPalette(const ColorPalette& palette) {
  this->beginResetModel();
  this->palette_ = palette;
  this->endResetModel();
}

}  // namespace hebo