// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/left_panel_button.h"

#include <QFont>
#include <QPainter>
#include <QPen>

namespace hebo {
namespace {

constexpr int kFixedWidth = 72;
constexpr int kFixedHeight = 54;

}  // namespace

LeftPanelButton::LeftPanelButton(const QString& icon_font, const QString& text, QWidget* parent)
    : QAbstractButton(parent), icon_font_(icon_font) {
  this->setText(text);
  this->setCheckable(true);
}

void LeftPanelButton::paintEvent(QPaintEvent* event) {
  Q_UNUSED(event);
  QPainter painter(this);

  QPen pen(painter.pen());
  if (!this->isChecked()) {
   if (this->underMouse()) {
     painter.fillRect(this->rect(), QColor("#5e5f60"));
   } else {
     painter.fillRect(this->rect(), QColor("#404142"));
   }
  } else {
    painter.fillRect(this->rect(), QColor("#222222"));
    pen.setColor(Qt::white);
    pen.setWidth(2);
    painter.setPen(pen);
    painter.drawLine(0, 0, 0, this->height());
  }

  // Now draw text
  if (!this->isChecked()) {
    pen.setColor("#a9acac");
  } else {
    pen.setColor(Qt::white);
  }
  painter.setPen(pen);
  QFont font(painter.font());
  font.setPixelSize(22);
  font.setFamilies({"element", "Noto Color Emoji"});
  painter.setFont(font);
  const int kTextFlag = Qt::AlignHCenter | Qt::AlignVCenter;
  const QRect icon_rect{0, 0, this->width(), 40};
  painter.drawText(icon_rect, kTextFlag, this->icon_font_);

  font.setPixelSize(10);
  painter.setFont(font);
  const QRect text_rect{0, 18, this->width(), this->height() - 16};
  painter.drawText(text_rect, kTextFlag, this->text());
}

QSize LeftPanelButton::sizeHint() const {
  return {kFixedWidth, kFixedHeight};
}

}  // namespace hebo