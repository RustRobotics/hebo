// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_SPIN_BOX_H_
#define HEBO_SRC_WIDGETS_SPIN_BOX_H_

#include <QSpinBox>

namespace hebo {

class SpinBox : public QSpinBox {
  Q_OBJECT
 public:
  explicit SpinBox(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_SPIN_BOX_H_
