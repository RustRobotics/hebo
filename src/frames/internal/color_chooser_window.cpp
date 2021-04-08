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

ColorChooserWindow::ColorChooserWindow(QWidget* parent) : QWidget(parent) {
  this->setWindowFlags(Qt::Popup);

  this->initUi();
  this->initSignals();
}

void ColorChooserWindow::initUi() {
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(0);
  this->setLayout(main_layout);

  auto* group = new QButtonGroup(this);
  auto* button_layout = new QHBoxLayout();
  button_layout->setContentsMargins(0, 0, 0, 0);
  button_layout->setSpacing(0);
  this->button_container_ = new QWidget();
  main_layout->addWidget(this->button_container_);
  this->button_container_->setLayout(button_layout);

  this->solid_button_ = new FlatButton(tr("Solid"));
  this->solid_button_->setCheckable(true);
  this->solid_button_->setChecked(true);
  this->gradient_button_ = new FlatButton(tr("Gradient"));
  this->gradient_button_->setCheckable(true);
  group->addButton(this->solid_button_);
  group->addButton(this->gradient_button_);
  button_layout->addWidget(this->solid_button_);
  button_layout->addWidget(this->gradient_button_);

  this->stacked_layout_ = new QStackedLayout();
  main_layout->addLayout(this->stacked_layout_);

  this->initSolidColor();
  this->initGradient();

  this->setFixedWidth(290);
}

void ColorChooserWindow::initSolidColor() {
  this->solid_page_ = new QWidget();
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(9, 9, 9, 9);
  main_layout->setSpacing(6);
  this->solid_page_->setLayout(main_layout);
  this->stacked_layout_->addWidget(this->solid_page_);

  auto* button_layout = new QHBoxLayout();
  button_layout->setSpacing(10);
  button_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->addLayout(button_layout);

  this->color_chooser_button_ = new FlatButton(tr("Color Picker"));
  this->color_chooser_button_->setIcon(QPixmap(kImageActionColorPicker));
  button_layout->addWidget(this->color_chooser_button_);
  this->transparent_button_ = new FlatButton(tr("Transparent"));
  this->transparent_button_->setIcon(QPixmap(kImageActionColorTransparent));
  button_layout->addWidget(this->transparent_button_);

  this->color_picker_ = new HSVColorPicker();
  main_layout->addWidget(this->color_picker_);

  auto* grid_layout = new QGridLayout();
  grid_layout->setContentsMargins(0, 0, 0, 0);
  grid_layout->setSpacing(10);
  main_layout->addLayout(grid_layout);
  this->color_line_edit_ = new ColorLineEdit();
  grid_layout->addWidget(this->color_line_edit_, 0, 0);
  this->r_line_edit_ = new ColorChannelLineEdit();
  grid_layout->addWidget(this->r_line_edit_, 0, 1);
  this->g_line_edit_ = new ColorChannelLineEdit();
  grid_layout->addWidget(this->g_line_edit_, 0, 2);
  this->b_line_edit_ = new ColorChannelLineEdit();
  grid_layout->addWidget(this->b_line_edit_, 0, 3);
  this->a_line_edit_ = new ColorChannelLineEdit();
  grid_layout->addWidget(this->a_line_edit_, 0, 4);

  auto* hex_label = new ColorLabel(tr("Hex"));
  grid_layout->addWidget(hex_label, 1, 0);
  auto* r_label = new ColorLabel(tr("R"));
  grid_layout->addWidget(r_label, 1, 1);
  auto* g_label = new ColorLabel(tr("G"));
  grid_layout->addWidget(g_label, 1, 2);
  auto* b_label = new ColorLabel(tr("B"));
  grid_layout->addWidget(b_label, 1, 3);
  this->a_label_ = new ColorLabel(tr("A"));
  grid_layout->addWidget(this->a_label_);

  main_layout->addWidget(new SeparatorLine(Qt::Horizontal));

  this->color_palette_list_view_ = new ColorPaletteListView();
  main_layout->addWidget(this->color_palette_list_view_);
}

void ColorChooserWindow::initGradient() {
  this->gradient_page_ = new QWidget();
  this->stacked_layout_->addWidget(this->gradient_page_);
  auto* main_layout = new QVBoxLayout();
  main_layout->setSpacing(0);
  main_layout->setContentsMargins(0, 0, 0, 0);
  this->gradient_page_->setLayout(main_layout);
}

void ColorChooserWindow::initSignals() {
  connect(this->solid_button_, &FlatButton::clicked, [=]() {
    this->stacked_layout_->setCurrentIndex(0);
  });
  connect(this->gradient_button_, &FlatButton::clicked, [=]() {
    this->stacked_layout_->setCurrentIndex(1);
  });

  connect(this->transparent_button_, &FlatButton::clicked, [=]() {
    this->solid_color_.setAlpha(0);
    this->updateColorEdit(this->solid_color_);
  });

  connect(this->color_palette_list_view_, &ColorPaletteListView::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_picker_, &HSVColorPicker::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->color_line_edit_, &ColorLineEdit::colorChanged,
          this, &ColorChooserWindow::updateColorEdit);
  connect(this->r_line_edit_, &ColorChannelLineEdit::valueChanged, [=](int value) {
    this->solid_color_.setRed(value);
    this->updateColorEdit(this->solid_color_);
  });
  connect(this->g_line_edit_, &ColorChannelLineEdit::valueChanged, [=](int value) {
    this->solid_color_.setGreen(value);
    this->updateColorEdit(this->solid_color_);
  });
  connect(this->b_line_edit_, &ColorChannelLineEdit::valueChanged, [=](int value) {
    this->solid_color_.setBlue(value);
    this->updateColorEdit(this->solid_color_);
  });
  connect(this->a_line_edit_, &ColorChannelLineEdit::valueChanged, [=](int value) {
    this->solid_color_.setAlpha(value);
    this->updateColorEdit(this->solid_color_);
  });
}

void ColorChooserWindow::setGradientVisible(bool visible) {
  this->button_container_->setVisible(visible);
  if (!visible) {
    this->solid_button_->setChecked(true);
  }
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
  this->r_line_edit_->setValue(color.red());
  this->g_line_edit_->setValue(color.green());
  this->b_line_edit_->setValue(color.blue());
  this->a_line_edit_->setValue(color.alpha());
  this->color_picker_->setColor(color);
  this->color_picker_->setPreviewColor(color);
}

void ColorChooserWindow::setEnableTransparent(bool enable) {
  this->transparent_button_->setVisible(enable);
  this->a_line_edit_->setVisible(enable);
  this->a_label_->setVisible(enable);
}

}  // namespace hebo
