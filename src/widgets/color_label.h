// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_COLOR_LABEL_H_
#define HEBO_SRC_WIDGETS_COLOR_LABEL_H_

#include <QLabel>

namespace hebo {

class ColorLabel : public QLabel {
  Q_OBJECT
 public:
  explicit ColorLabel(QWidget* parent = nullptr);
  explicit ColorLabel(const QString& text, QWidget* parent = nullptr);

 protected:
  void paintEvent(QPaintEvent* event) override;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_COLOR_LABEL_H_
