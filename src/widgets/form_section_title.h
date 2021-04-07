// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_FORM_SECTION_TITLE_H_
#define HEBO_SRC_WIDGETS_FORM_SECTION_TITLE_H_

#include <QLabel>

namespace hebo {

class FormSectionTitle : public QLabel {
  Q_OBJECT
 public:
  explicit FormSectionTitle(const QString& text, QWidget* parent = nullptr);
  ~FormSectionTitle() override = default;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_FORM_SECTION_TITLE_H_
