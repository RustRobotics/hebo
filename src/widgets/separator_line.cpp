// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/separator_line.h"

#include <QPainter>
#include <QPainterPath>

namespace hebo {
namespace {

constexpr int kFixedSize = 12;

}  // namespace

SeparatorLine::SeparatorLine(Qt::Orientation orientation, QWidget* parent)
    : QFrame(parent), orientation_(orientation) {
  if (orientation_ == Qt::Horizontal) {
    this->setFixedHeight(kFixedSize);
  } else {
    this->setFixedWidth(kFixedSize);
  }
}

void SeparatorLine::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event)
  QPainter painter(this);
  QPen pen(painter.pen());
  pen.setColor(QColor(195, 195, 195));
  pen.setWidth(4);
  if (this->orientation_ == Qt::Horizontal) {
    int y = this->height() / 2;
    painter.drawLine(0, y, this->width(), y);
  } else {
    int x = this->width() / 2;
    painter.drawLine(x, 0, x, this->height());
  }
}

}  // namespace hebo