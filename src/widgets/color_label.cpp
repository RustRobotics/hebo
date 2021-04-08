// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/color_label.h"

#include <QPainter>

namespace hebo {

ColorLabel::ColorLabel(QWidget* parent) : QLabel(parent) {

}

ColorLabel::ColorLabel(const QString& text, QWidget* parent) : QLabel(text, parent) {

}

void ColorLabel::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event)
  QPainter painter(this);
  QPen pen = painter.pen();
  pen.setColor(QColor(34, 34, 34));
  painter.setPen(pen);
  QFont font(painter.font());
  font.setPixelSize(11);
  painter.setFont(font);

  constexpr int kTextFlags = Qt::AlignCenter;
  painter.drawText(this->rect(), kTextFlags, this->text());
}

}  // namespace hebo