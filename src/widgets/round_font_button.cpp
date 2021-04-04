// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/round_font_button.h"

#include <QFont>

#include "resources/fonts/fonts.h"

namespace hebo {

RoundFontButton::RoundFontButton(const QString& text, QWidget* parent) : QPushButton(text, parent) {
  QFont font(this->font());
  font.setFamilies({
    "Noto Color Emoji",
    "GSUB"
  });
  font.setPixelSize(12);
  this->setFont(font);
  
  this->resize(24, 24);
  this->setStyleSheet("border-radius: 12px;");
}

}  // namespace hebo