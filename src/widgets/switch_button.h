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
 public:
  explicit SwitchButton(QWidget* parent = nullptr);
  ~SwitchButton() override = default;

  [[nodiscard]] QSize sizeHint() const override;

  [[nodiscard]] int offset() const { return this->x_; }

 public slots:
  void setOffset(int offset) {
    this->x_ = offset;
    this->update();
  }

 protected:
  void paintEvent(QPaintEvent*) override;

  void mouseReleaseEvent(QMouseEvent*) override;

  void enterEvent(QEvent*) override;

  void checkStateSet() override;

  void resizeEvent(QResizeEvent* event) override;

 private:
  void resetAnimation(bool is_checked);

  int x_;
  int y_;
  int height_;
  int margin_;
  QBrush thumb_;
  QBrush brush_;
  qreal opacity_;
  QPropertyAnimation* animation_;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_SWITCH_BUTTON_H_
