// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_COLOR_CHOOSER_BUTTON_H
#define HEBO_SRC_WIDGETS_COLOR_CHOOSER_BUTTON_H

#include <QWidget>

namespace hebo {

class ColorChooserButton : public QWidget {
  Q_OBJECT
  Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY colorChanged)

 public:
  explicit ColorChooserButton(QWidget* parent = nullptr);

  [[nodiscard]] const QColor& color() const { return this->color_; }

 public slots:
  void setColor(const QColor& color);

 signals:
  void colorChanged(const QColor& color);

 protected:
  void paintEvent(QPaintEvent* event) override;

  void mousePressEvent(QMouseEvent* e) override;

  void enterEvent(QEvent* event) override;

  void leaveEvent(QEvent* event) override;

 private:
  void initUi();

  void initSignals();

  QColor color_;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_COLOR_CHOOSER_BUTTON_H
