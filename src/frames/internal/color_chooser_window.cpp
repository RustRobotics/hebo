// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/color_chooser_window.h"

#include <QButtonGroup>
#include <QDebug>
#include <QGridLayout>
#include <QVBoxLayout>

namespace hebo {

ColorChooserWindow::ColorChooserWindow(QWidget* parent) : QDialog(parent) {
  this->initUi();
  this->initSignals();
  this->setModal(true);
}

void ColorChooserWindow::initUi() {
  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  this->color_picker_ = new HSVColorPicker();
  main_layout->addWidget(this->color_picker_);

  auto* grid_layout = new QGridLayout();
  grid_layout->setContentsMargins(0, 0, 0, 0);
  grid_layout->setSpacing(10);
  main_layout->addLayout(grid_layout);
  this->color_line_edit_ = new ColorLineEdit();
  grid_layout->addWidget(this->color_line_edit_, 0, 0);

  this->color_palette_list_view_ = new ColorPaletteListView();
  main_layout->addWidget(this->color_palette_list_view_);

  this->close_button_ = new QPushButton(tr("Close"));
  main_layout->addWidget(this->close_button_, 0, Qt::AlignRight);
}

void ColorChooserWindow::initSignals() {
  connect(this->color_palette_list_view_, &ColorPaletteListView::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_picker_, &HSVColorPicker::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_line_edit_, &ColorLineEdit::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);

  connect(this->close_button_, &QPushButton::clicked,
          this, &ColorChooserWindow::close);
}

void ColorChooserWindow::setSolidColorPalette(const ColorPalette& palette) {
  this->color_palette_list_view_->setColorPalette(palette);
}

void ColorChooserWindow::updateColorEdit(const QColor& color) {
  this->setColor(color);
  emit this->colorChanged(color);
}

void ColorChooserWindow::setColor(const QColor& color) {
  this->solid_color_ = color;
  this->color_line_edit_->setColor(color);
  this->color_picker_->setColor(color);
  this->color_picker_->setPreviewColor(color);
}

}  // namespace hebo
