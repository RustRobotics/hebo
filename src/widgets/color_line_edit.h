// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_COLOR_LINE_EDIT_H
#define HEBO_SRC_WIDGETS_COLOR_LINE_EDIT_H

#include <QLineEdit>

namespace hebo {

class ColorLineEdit : public QLineEdit {
  Q_OBJECT

  Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY colorChanged)

 public:
  explicit ColorLineEdit(QWidget* parent = nullptr);

  [[nodiscard]] QColor color() const;

 public slots:
  void setColor(const QColor& color);

 signals:
  void colorChanged(const QColor& color);

 private slots:
  void onTextChanged();
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_COLOR_LINE_EDIT_H
