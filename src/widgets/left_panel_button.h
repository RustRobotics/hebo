// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_LEFT_PANEL_BUTTON_H_
#define HEBO_SRC_WIDGETS_LEFT_PANEL_BUTTON_H_

#include <QAbstractButton>

namespace hebo {

class LeftPanelButton : public QAbstractButton {
  Q_OBJECT
 public:
  LeftPanelButton(const QString& icon_font, const QString& text, QWidget* parent = nullptr);

  [[nodiscard]] QSize sizeHint() const override;

 protected:
  void paintEvent(QPaintEvent* event) override;

 private:
  QString icon_font_;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_LEFT_PANEL_BUTTON_H_
