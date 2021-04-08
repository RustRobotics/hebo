// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_MODELS_COLOR_PALETTE_MODEL_H_
#define HEBO_SRC_WIDGETS_MODELS_COLOR_PALETTE_MODEL_H_

#include <QAbstractListModel>

#include "formats/color_palette.h"

namespace hebo {

class ColorPaletteModel : public QAbstractListModel {
  Q_OBJECT
 public:
  explicit ColorPaletteModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  void setPalette(const ColorPalette& palette);

 private:
  ColorPalette palette_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_MODELS_COLOR_PALETTE_MODEL_H_
