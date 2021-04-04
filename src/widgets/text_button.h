// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_TEXT_BUTTON_H_
#define HEBO_SRC_WIDGETS_TEXT_BUTTON_H_

#include <QLabel>

namespace hebo {

class TextButton : public QLabel {
  Q_OBJECT
 public:
  explicit TextButton(const QString& text, QWidget* parent = nullptr);

 signals:
  void clicked();

 protected:
  void mousePressEvent(QMouseEvent* ev) override;

  void enterEvent(QEvent* event) override;

  void leaveEvent(QEvent* event) override;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_TEXT_BUTTON_H_
