// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_COLOR_PALETTE_LIST_VIEW_H_
#define HEBO_SRC_WIDGETS_COLOR_PALETTE_LIST_VIEW_H_

#include <QListView>

#include "formats/color_palette.h"
#include "widgets/models/color_palette_model.h"

namespace hebo {

class ColorPaletteListView : public QListView {
  Q_OBJECT
 public:
  explicit ColorPaletteListView(QWidget* parent = nullptr);

  void setColorPalette(const ColorPalette& palette);

 signals:
  void colorChanged(const QColor& color);

 private slots:
  void onItemClicked(const QModelIndex& index);

 private:
  void initUi();
  void initSignals();

  ColorPaletteModel* model_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_COLOR_PALETTE_LIST_VIEW_H_
