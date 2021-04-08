// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/delegates/color_palette_delegate.h"

#include <QPainter>
#include <QPainterPath>

namespace hebo {

ColorPaletteDelegate::ColorPaletteDelegate(QObject* parent) : QStyledItemDelegate(parent) {

}

void ColorPaletteDelegate::paint(QPainter* painter,
                                 const QStyleOptionViewItem& option,
                                 const QModelIndex& index) const {
  const QColor color = index.data(Qt::DecorationRole).value<QColor>();
  painter->fillRect(option.rect, color);

  painter->setRenderHints(QPainter::Antialiasing);
  QPainterPath path;
  path.addRoundedRect(option.rect, 4.3, 4.3);
  QPen pen;
  pen.setWidthF(0.3);
  pen.setColor(QColor(0, 0, 0));
  painter->setPen(pen);
  painter->fillPath(path, color);
  painter->drawPath(path);
}

QSize ColorPaletteDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const {
  Q_UNUSED(option);
  Q_UNUSED(index);
  return {24, 24};
}

}  // namespace hebo