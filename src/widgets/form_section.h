// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_FORM_SECTION_H_
#define HEBO_SRC_WIDGETS_FORM_SECTION_H_

#include <QFrame>

namespace hebo {

class FormSection : public QFrame {
  Q_OBJECT
 public:
  explicit FormSection(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_FORM_SECTION_H_
