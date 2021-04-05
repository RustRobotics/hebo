// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_SWITCH_BUTTON_H_
#define HEBO_SRC_WIDGETS_SWITCH_BUTTON_H_

#include <QtWidgets>

namespace hebo {

class SwitchButton : public QAbstractButton {
 Q_OBJECT
  Q_PROPERTY(int offset READ offset WRITE setOffset)
  Q_PROPERTY(bool checked READ checked WRITE setChecked NOTIFY checkedChanged);
 public:
  explicit SwitchButton(QWidget* parent = nullptr);
  ~SwitchButton() override = default;

  [[nodiscard]] QSize sizeHint() const override;

  [[nodiscard]] bool checked() const { return this->checked_; }

  [[nodiscard]] int offset() const { return this->x_; }

 public slots:
  void setChecked(bool checked) {
    this->checked_ = checked;
    this->update();
  };

  void setOffset(int offset) {
    this->x_ = offset;
    this->update();
  }

 signals:
  void checkedChanged(bool checked);

 protected:
  void paintEvent(QPaintEvent*) override;

  void mouseReleaseEvent(QMouseEvent*) override;

  void enterEvent(QEvent*) override;

 private:
  bool checked_;
  int x_;
  int y_;
  int height_;
  int margin_;
  QBrush thumb_;
  QBrush brush_;
  QPropertyAnimation* animation_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_SWITCH_BUTTON_H_
