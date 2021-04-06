// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/switch_button.h"

namespace hebo {
namespace {

constexpr const char* kDefaultBrush =  "#009688";
constexpr const char* kDefaultThumb = "#d5d5d5";
constexpr const qreal kCheckedOpacity = 0.5;
constexpr const qreal kUncheckedOpacity = 0.38;

}  // namespace

SwitchButton::SwitchButton(QWidget* parent)
    : QAbstractButton(parent),
      x_(8),
      y_(8),
      height_(16),
      margin_(4),
      thumb_(kDefaultThumb),
      brush_(Qt::black),
      opacity_(kUncheckedOpacity),
      animation_(new QPropertyAnimation(this, "offset", this)) {
  this->setCheckable(true);
  this->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Fixed);
}

void SwitchButton::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event);

  QPainter p(this);
  p.setPen(Qt::NoPen);
  if (isEnabled()) {
    p.setBrush(this->brush_);
    p.setOpacity(this->opacity_);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.drawRoundedRect(QRect(margin_, margin_, width() - 2 * margin_,
                            height() - 2 * margin_), 8.0, 8.0);
    p.setBrush(this->thumb_);
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
  QAbstractButton::mouseReleaseEvent(event);
  if (event->button() & Qt::LeftButton) {
    this->resetAnimation(this->isChecked());
  }
}

void SwitchButton::enterEvent(QEvent* event) {
  this->setCursor(Qt::PointingHandCursor);
  QAbstractButton::enterEvent(event);
}

QSize SwitchButton::sizeHint() const {
  return QSize(static_cast<int>(2.4 * (height_ + margin_)), height_ + 2 * margin_);
}

void SwitchButton::checkStateSet() {
  QAbstractButton::checkStateSet();
  this->resetAnimation(this->isChecked());
}

void SwitchButton::resetAnimation(bool is_checked) {
  if (is_checked) {
    this->brush_.setColor(kDefaultBrush);
    this->thumb_.setColor(this->brush_.color());
    this->opacity_ = kCheckedOpacity;

    this->animation_->setStartValue(this->height_ / 2);
    this->animation_->setEndValue(width() - this->height_);
    this->animation_->setDuration(120);
    this->animation_->start();
  } else {
    this->brush_.setColor(Qt::black);
    this->thumb_.setColor(kDefaultThumb);
    this->opacity_ = kUncheckedOpacity;

    this->animation_->setStartValue(this->x_);
    this->animation_->setEndValue(this->height_ / 2);
    this->animation_->setDuration(120);
    this->animation_->start();
  }
}

void SwitchButton::resizeEvent(QResizeEvent* event) {
  QWidget::resizeEvent(event);
  if (this->isChecked()) {
    this->setOffset(width() - this->height_);
  } else {
    this->setOffset(this->height_ / 2);
  }
}

}  // namespace hebo