// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_DELEGATES_COLOR_PALETTE_DELEGATE_H_
#define HEBO_SRC_WIDGETS_DELEGATES_COLOR_PALETTE_DELEGATE_H_

#include <QStyledItemDelegate>

namespace hebo {

class ColorPaletteDelegate : public QStyledItemDelegate{
  Q_OBJECT
 public:
  explicit ColorPaletteDelegate(QObject* parent = nullptr);

  void paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const override;

  QSize sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const override;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_DELEGATES_COLOR_PALETTE_DELEGATE_H_
