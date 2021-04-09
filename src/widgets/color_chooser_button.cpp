// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/color_chooser_button.h"

#include <QDebug>
#include <QPainter>
#include <QPainterPath>

#include "resources/images/images.h"

namespace hebo {

ColorChooserButton::ColorChooserButton(QWidget* parent) : QWidget(parent) {
  this->setFixedSize(48, 22);
  this->setMouseTracking(true);
}

void ColorChooserButton::setColor(const QColor& color) {
  this->color_ = color;
  this->update();
}

void ColorChooserButton::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event);
  QPainter painter(this);
  painter.setRenderHints(QPainter::Antialiasing);
  QPainterPath path;
  path.addRoundedRect(this->rect(), 4.3, 4.3);
  QPen pen;
  pen.setWidthF(0.3);
  pen.setColor(QColor(0, 0, 0));
  painter.setPen(pen);
  painter.fillPath(path, this->color_);
  painter.drawPath(path);
}

void ColorChooserButton::mousePressEvent(QMouseEvent* event) {
  Q_UNUSED(event);
  emit this->clicked();
}

void ColorChooserButton::enterEvent(QEvent* event) {
  this->update();
  QWidget::enterEvent(event);
}

void ColorChooserButton::leaveEvent(QEvent* event) {
  this->update();
  QWidget::leaveEvent(event);
}

}  // namespace hebo