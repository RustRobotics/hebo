// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/form_section_title.h"

namespace hebo {

FormSectionTitle::FormSectionTitle(const QString& text, QWidget* parent) : QLabel(text, parent) {
  // style: form-section-title.css
}

}  // namespace hebo