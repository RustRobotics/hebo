// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/form_section_title.h"

#include <QFont>

namespace hebo {

FormSectionTitle::FormSectionTitle(const QString& text, QWidget* parent) : QLabel(text, parent) {
  QFont font(this->font());
  font.setPixelSize(22);
  this->setFont(font);
}

}  // namespace hebo