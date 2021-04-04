// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/text_button.h"

#include <QCursor>
#include <QStyle>

#include "base/file.h"
#include "resources/styles/styles.h"

namespace hebo {

TextButton::TextButton(const QString& text, QWidget* parent) : QLabel(text, parent) {
}

void TextButton::mousePressEvent(QMouseEvent* ev) {
  QLabel::mousePressEvent(ev);
  emit this->clicked();
}

void TextButton::enterEvent(QEvent* event) {
  QWidget::enterEvent(event);
  this->setCursor(QCursor(Qt::PointingHandCursor));
}

void TextButton::leaveEvent(QEvent* event) {
  QWidget::leaveEvent(event);
  this->unsetCursor();
}

}  // namespace hebo