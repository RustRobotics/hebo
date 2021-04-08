// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_COLOR_CHOOSER_WINDOW_H_
#define HEBO_SRC_FRAMES_COLOR_CHOOSER_WINDOW_H_

#include <QStackedLayout>
#include <QWidget>

#include "formats/color_palette.h"
#include "widgets/color_channel_line_edit.h"
#include "widgets/color_label.h"
#include "widgets/color_line_edit.h"
#include "widgets/color_palette_list_view.h"
#include "widgets/flat_button.h"
#include "widgets/hsv_color_picker.h"

namespace hebo {

class ColorChooserWindow : public QWidget {
 Q_OBJECT
 Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY colorChanged);

 public:
  explicit ColorChooserWindow(QWidget* parent = nullptr);

  void setSolidColorPalette(const ColorPalette& palette);

  [[nodiscard]] const QColor& color() const { return this->solid_color_; }

 public slots:
  void setColor(const QColor& color);

 signals:
  void colorChanged(const QColor& color);

  void lostFocus();

 protected:
  void focusOutEvent(QFocusEvent* event) override;

 private slots:
  void updateColorEdit(const QColor& color);

 private:
  void initUi();
  void initSignals();

  QWidget* solid_page_{nullptr};
  QColor solid_color_{};
  ColorPaletteListView* color_palette_list_view_{nullptr};
  HSVColorPicker* color_picker_{nullptr};
  ColorLineEdit* color_line_edit_{nullptr};

  ColorLabel* a_label_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_COLOR_CHOOSER_WINDOW_H_
