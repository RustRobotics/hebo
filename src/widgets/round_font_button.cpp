// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/round_font_button.h"

#include <QFont>

#include "base/file.h"
#include "resources/styles/styles.h"

namespace hebo {

RoundFontButton::RoundFontButton(const QString& text, QWidget* parent) : QPushButton(text, parent) {
  this->setFixedSize(46, 46);
  this->setStyleSheet(readTextFile(kStyleRoundFontButton));
}

}  // namespace hebo