// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/form_section_title.h"

#include "base/file.h"
#include "resources/styles/styles.h"
#include "form_section_title.h"

namespace hebo {

hebo::FormSectionTitle::FormSectionTitle(const QString& text, QWidget* parent)
    : QLabel(text, parent) {
  this->setStyleSheet(readTextFile(kStyleFormSectionTitle));
}

}  // namespace hebo