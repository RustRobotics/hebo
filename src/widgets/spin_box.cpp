// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/spin_box.h"

#include "base/file.h"
#include "resources/styles/styles.h"

namespace hebo {

SpinBox::SpinBox(QWidget* parent) : QSpinBox(parent) {
  this->setStyleSheet(readTextFile(kStyleSpinBox));
}

}  // namespace hebo