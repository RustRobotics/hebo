// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_SEPARATOR_LINE_H_
#define HEBO_SRC_WIDGETS_SEPARATOR_LINE_H_

#include <QFrame>

namespace hebo {

class SeparatorLine : public QFrame {
  Q_OBJECT
 public:
  explicit SeparatorLine(Qt::Orientation orientation, QWidget* parent = nullptr);

 protected:
  void paintEvent(QPaintEvent* event) override;

 private:
  Qt::Orientation orientation_;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_SEPARATOR_LINE_H_
