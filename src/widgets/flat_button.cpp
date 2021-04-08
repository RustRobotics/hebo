// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/flat_button.h"

#include "base/file.h"
#include "resources/styles/styles.h"

namespace hebo {

FlatButton::FlatButton(const QString& text, QWidget* parent) : QPushButton(text, parent) {
  this->setStyleSheet(readTextFile(kStyleFlatButton));
}

}  // namespace hebo