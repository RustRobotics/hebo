// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/switch_button.h"

namespace hebo {

SwitchButton::SwitchButton(QWidget* parent)
    : QAbstractButton(parent),
      checked_(false),
      x_(8),
      y_(8),
      height_(16),
      margin_(4),
      thumb_("#d5d5d5"),
      brush_(QColor("#009688")),
      animation_(new QPropertyAnimation(this, "offset", this)) {
}

void SwitchButton::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event);

  QPainter p(this);
  p.setPen(Qt::NoPen);
  if (isEnabled()) {
    p.setBrush(checked_ ? brush_ : Qt::black);
    p.setOpacity(checked_ ? 0.5 : 0.38);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.drawRoundedRect(QRect(margin_, margin_, width() - 2 * margin_,
                            height() - 2 * margin_), 8.0, 8.0);
    p.setBrush(thumb_);
    p.setOpacity(1.0);
    p.drawEllipse(QRectF(x_ - (height_ / 2.0), y_ - (height_ / 2.0),
                         height(), height()));
  } else {
    p.setBrush(Qt::black);
    p.setOpacity(0.12);
    p.drawRoundedRect(QRect(margin_, margin_, width() - 2 * margin_,
                            height() - 2 * margin_), 8.0, 8.0);
    p.setOpacity(1.0);
    p.setBrush(QColor("#BDBDBD"));
    p.drawEllipse(QRectF(x_ - (height_ / 2.0), y_ - (height_ / 2.0),
                         height(), height()));
  }
}

void SwitchButton::mouseReleaseEvent(QMouseEvent* event) {
  if (event->button() & Qt::LeftButton) {
    checked_ = !checked_;
    thumb_ = checked_ ? brush_ : QBrush("#d5d5d5");
    if (checked_) {
      animation_->setStartValue(height_ / 2);
      animation_->setEndValue(width() - height_);
      animation_->setDuration(120);
      animation_->start();
    } else {
      animation_->setStartValue(x_);
      animation_->setEndValue(height_ / 2);
      animation_->setDuration(120);
      animation_->start();
    }
  }
  QAbstractButton::mouseReleaseEvent(event);
}

void SwitchButton::enterEvent(QEvent* event) {
  setCursor(Qt::PointingHandCursor);
  QAbstractButton::enterEvent(event);
}

QSize SwitchButton::sizeHint() const {
  return QSize(static_cast<int>(2.4 * (height_ + margin_)), height_ + 2 * margin_);
}

}  // namespace hebo