// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/color_chooser_window.h"

#include <QButtonGroup>
#include <QDebug>
#include <QGridLayout>
#include <QVBoxLayout>

#include "resources/images/images.h"
#include "widgets/color_chooser_button.h"
#include "widgets/separator_line.h"

namespace hebo {

ColorChooserWindow::ColorChooserWindow(QWidget* parent) : QDialog(parent) {
  this->initUi();
  this->initSignals();
}

void ColorChooserWindow::initUi() {
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(6);
  this->setLayout(main_layout);
  this->setFixedWidth(290);

  this->color_picker_ = new HSVColorPicker();
  main_layout->addWidget(this->color_picker_);

  auto* grid_layout = new QGridLayout();
  grid_layout->setContentsMargins(0, 0, 0, 0);
  grid_layout->setSpacing(10);
  main_layout->addLayout(grid_layout);
  this->color_line_edit_ = new ColorLineEdit();
  grid_layout->addWidget(this->color_line_edit_, 0, 0);
  main_layout->addWidget(new SeparatorLine(Qt::Horizontal));

  this->color_palette_list_view_ = new ColorPaletteListView();
  main_layout->addWidget(this->color_palette_list_view_);
}

void ColorChooserWindow::initSignals() {
  connect(this->color_palette_list_view_, &ColorPaletteListView::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_picker_, &HSVColorPicker::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_line_edit_, &ColorLineEdit::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
}

void ColorChooserWindow::setSolidColorPalette(const ColorPalette& palette) {
  this->color_palette_list_view_->setColorPalette(palette);
}

void ColorChooserWindow::updateColorEdit(const QColor& color) {
  this->setColor(color);
  emit this->colorChanged(color);
}

void ColorChooserWindow::focusOutEvent(QFocusEvent* event) {
  QWidget::focusOutEvent(event);
  emit this->lostFocus();
}

void ColorChooserWindow::setColor(const QColor& color) {
  this->solid_color_ = color;
  this->color_line_edit_->setColor(color);
  this->color_picker_->setColor(color);
  this->color_picker_->setPreviewColor(color);
}

}  // namespace hebo
