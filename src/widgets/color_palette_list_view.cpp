// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/color_palette_list_view.h"

#include "widgets/delegates/color_palette_delegate.h"

namespace hebo {

ColorPaletteListView::ColorPaletteListView(QWidget* parent) : QListView(parent) {
  this->initUi();
  this->initSignals();
}

void ColorPaletteListView::initUi() {
  this->setViewMode(QListView::ViewMode::IconMode);
  this->setFrameShape(QFrame::Shape::NoFrame);
  this->setSpacing(6);
  this->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
  this->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
  this->setAcceptDrops(false);
  this->setWrapping(true);
  this->setUniformItemSizes(true);
  this->setResizeMode(QListView::ResizeMode::Adjust);
  this->setFlow(QListView::LeftToRight);
  this->setSelectionMode(QListView::SingleSelection);

  auto* delegate = new ColorPaletteDelegate(this);
  this->setItemDelegate(delegate);

  this->model_ = new ColorPaletteModel(this);
  this->setModel(this->model_);
}

void ColorPaletteListView::initSignals() {
  connect(this, &ColorPaletteListView::clicked,
          this, &ColorPaletteListView::onItemClicked);
}

void ColorPaletteListView::setColorPalette(const ColorPalette& palette) {
  this->model_->setPalette(palette);
}

void ColorPaletteListView::onItemClicked(const QModelIndex& index) {
  const QColor color = index.data(Qt::DecorationRole).value<QColor>();
  emit this->colorChanged(color);
}

}  // namespace hebo