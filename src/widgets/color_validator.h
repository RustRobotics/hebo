// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_COLOR_VALIDATOR_H
#define HEBO_SRC_WIDGETS_COLOR_VALIDATOR_H

#include <QValidator>

namespace hebo {

class ColorValidator : public QValidator {
  Q_OBJECT
 public:
  explicit ColorValidator(QObject* parent = nullptr);

  State validate(QString& input, int& pos) const override;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_COLOR_VALIDATOR_H
