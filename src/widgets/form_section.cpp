// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/form_section.h"

#include "base/file.h"
#include "resources/styles/styles.h"

namespace hebo {

FormSection::FormSection(QWidget* parent) : QFrame(parent) {
  this->setStyleSheet(readTextFile(kStyleFormSection));
}

}  // namespace hebo