// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/color_line_edit.h"

#include <QDebug>

#include "widgets/color_validator.h"

namespace hebo {

ColorLineEdit::ColorLineEdit(QWidget* parent) : QLineEdit(parent) {
  QValidator* validator = new ColorValidator(this);
  this->setValidator(validator);

  this->setToolTip(tr(
      "Color name may be in one of these formats:\n"
      "- #RGB\n"
      "- #RRGGBB\n"
      "- #AARRGGBB\n"
      "- #RRRGGGBBB\n"
      "- #RRRRGGGGBBBB\n"
      "- SVG color keyword names provided by the World Wide Web Consortium\n"
      "- transparent"));

  connect(this, &ColorLineEdit::editingFinished,
          this, &ColorLineEdit::onTextChanged);

  // style: color-line-edit.css
}

QColor ColorLineEdit::color() const {
  return {this->text()};
}

void ColorLineEdit::setColor(const QColor& color) {
  this->setText(color.name());
}

void ColorLineEdit::onTextChanged() {
  emit this->colorChanged(QColor(this->text()));
}

}  // namespace hebo