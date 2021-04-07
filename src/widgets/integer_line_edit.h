// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_NUMBER_LINE_EDIT_H
#define HEBO_SRC_WIDGETS_NUMBER_LINE_EDIT_H

#include <QIntValidator>
#include <QLineEdit>

namespace hebo {

class IntegerLineEdit : public QLineEdit {
  Q_OBJECT

  Q_PROPERTY(int value READ value WRITE setValue NOTIFY valueChanged)

 public:
  explicit IntegerLineEdit(QWidget* parent = nullptr);

  void setRange(int min, int max);

  [[nodiscard]] int value() const;

 public slots:
  void setValue(int integer);

 signals:
  void valueChanged(int integer);

 protected:
  void keyPressEvent(QKeyEvent* event) override;

 private slots:
  void onTextChanged(const QString& text);

 private:
  bool validateInteger(int integer);

  QIntValidator* validator_;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_NUMBER_LINE_EDIT_H
