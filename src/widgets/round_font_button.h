// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_WIDGETS_ROUND_FONT_BUTTON_H_
#define HEBOUI_SRC_WIDGETS_ROUND_FONT_BUTTON_H_

#include <QPushButton>

namespace hebo {

class RoundFontButton : public QPushButton {
  Q_OBJECT
 public:
  explicit RoundFontButton(const QString& text, QWidget* parent = nullptr);
  ~RoundFontButton() override = default;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_WIDGETS_ROUND_FONT_BUTTON_H_
