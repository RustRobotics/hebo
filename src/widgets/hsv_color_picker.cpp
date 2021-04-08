// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/hsv_color_picker.h"

#include <QDebug>
#include <QMouseEvent>
#include <QPainter>

#include "resources/images/images.h"

namespace hebo {
namespace {

constexpr int kMinHue = 0;
constexpr int kMaxHue = 359;
constexpr int kMinSaturation = 0;
constexpr int kMaxSaturation = 255;
constexpr int kMinValue = 0;
constexpr int kMaxValue = 255;

constexpr int kHueSliderHeight = 16;
constexpr int kHueSliderWidth = 224;
constexpr qreal kHueScale = kHueSliderWidth * 1.0f / kMaxHue;
constexpr int kHueSliderPaddingTop = 8;
constexpr int kPreviewWidth = 24;

}  // namespace

HSVColorPicker::HSVColorPicker(QWidget* parent) : QWidget(parent) {
  this->initUi();
}

void HSVColorPicker::setColor(const QColor& color) {
  this->color_ = color;
  this->update();
}

void HSVColorPicker::setPreviewColor(const QColor& color) {
  this->preview_color_ = color;
  this->update();
}

void HSVColorPicker::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event);

  QPainter painter(this);
  QColor color;
  QPen pen;
  pen.setWidth(1);
  painter.setPen(pen);

  // Draw saturation-value panel.
  // x: saturation, 0 -> 255
  // y: value, 255 -> 0
  for (int value = kMinValue; value <= kMaxValue; ++value) {
    for (int saturation = kMinSaturation; saturation <= kMaxSaturation; ++saturation) {
      color.setHsv(this->color_.hue(), saturation, value);
      pen.setColor(color);
      painter.setPen(pen);
      painter.drawPoint(saturation, kMaxValue - value);
    }
  }

  // Draw cross-hair indicator.
  const bool is_black_color = this->color_.value() < kMaxValue / 2;
  const int kPixmapX = this->color_.saturation() - 8;
  const int kPixmapY = kMaxValue - this->color_.value() - 8;
  painter.drawPixmap(kPixmapX, kPixmapY,
                     is_black_color ? QPixmap(kImageWhitePlus) : QPixmap(kImageBlackPlus));

  // Draw preview icon
  painter.fillRect(this->preview_rect_, this->preview_color_);

  // Draw hue slider. 355 -> 0
  painter.scale(kHueScale, 1);
  for (int hue = kMinHue; hue <= kMaxHue; ++hue) {
    color.setHsv(hue, kMaxSaturation, kMaxValue);
    pen.setColor(color);
    painter.setPen(pen);
    painter.drawLine(kMaxHue - hue, this->hue_rect_.top(),
                     kMaxHue - hue, this->hue_rect_.bottom());
  }

  // Draw indicators.
  pen.setColor(QColor(32, 32, 32));
  pen.setWidth(2);
  painter.setPen(pen);
  painter.drawLine(kMaxHue - this->color_.hue(), this->hue_rect_.top() - 1,
                   kMaxHue - this->color_.hue(), this->hue_rect_.bottom() + 2);
}

void HSVColorPicker::mousePressEvent(QMouseEvent* event) {
  if (this->hue_scaled_rect_.contains(event->pos())) {
    this->mouse_press_state_ = MousePressState::kHuePressed;
    this->updateHue(event->pos());
  } else if (this->sv_rect_.contains(event->pos())) {
    this->mouse_press_state_ = MousePressState::kSvPressed;
    this->updateSaturationValue(event->pos());
  } else {
    this->mouse_press_state_ = MousePressState::kNone;
  }
}

void HSVColorPicker::resizeEvent(QResizeEvent* event) {
  QWidget::resizeEvent(event);
  this->sv_rect_.setRect(0, 0, kMaxSaturation, kMaxValue);
  this->hue_rect_.setRect(0, this->sv_rect_.bottom() + kHueSliderPaddingTop,
                          kMaxHue, kHueSliderHeight);
  this->hue_scaled_rect_.setRect(this->hue_rect_.left(), this->hue_rect_.top(),
                                 kMaxHue * kHueScale, kHueSliderHeight);
  this->preview_rect_.setRect(this->hue_scaled_rect_.right() + kHueSliderPaddingTop,
                              this->hue_rect_.top(),
                              kPreviewWidth, kHueSliderHeight);
}

void HSVColorPicker::mouseMoveEvent(QMouseEvent* event) {
  switch (this->mouse_press_state_) {
    case MousePressState::kHuePressed: {
      this->updateHue(event->pos());
      break;
    }
    case MousePressState::kSvPressed: {
      this->updateSaturationValue(event->pos());
      break;
    }
    default: {
    }
  }

  QWidget::mouseMoveEvent(event);
}

void HSVColorPicker::mouseReleaseEvent(QMouseEvent* event) {
  this->mouse_press_state_ = MousePressState::kNone;
  QWidget::mouseReleaseEvent(event);
}

void HSVColorPicker::initUi() {
  this->setMinimumSize(kMaxValue,
                       kMaxValue + kHueSliderPaddingTop + kHueSliderHeight);
}

void HSVColorPicker::updateHue(const QPoint& pos) {
  const int delta = (pos.x() - this->hue_rect_.x()) / kHueScale;
  int hue = kMaxHue - delta;
  hue = qMin(hue, kMaxHue);
  hue = qMax(hue, kMinHue);
  this->color_.setHsv(hue, this->color_.saturation(), this->color_.value());
  this->preview_color_ = this->color_;
  this->update();
  emit this->colorChanged(this->color_);
}

void HSVColorPicker::updateSaturationValue(const QPoint& pos) {
  int saturation = pos.x();
  saturation = qMin(saturation, kMaxSaturation);
  saturation = qMax(saturation, kMinSaturation);
  int value = kMaxValue - pos.y();
  value = qMin(value, kMaxValue);
  value = qMax(value, kMinValue);
  this->color_.setHsv(this->color_.hue(), saturation, value);
  this->preview_color_ = this->color_;
  this->update();
  emit this->colorChanged(this->color_);
}

}  // namespace hebo