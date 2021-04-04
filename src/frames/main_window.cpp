// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/main_window.h"

#include <QDebug>

namespace hebo {

MainWindow::MainWindow(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
}

void MainWindow::initUi() {
  auto* main_layout = new QHBoxLayout();
  main_layout->setSpacing(0);
  main_layout->setContentsMargins(0, 0, 0, 0);
  this->setLayout(main_layout);

  this->left_panel_ = new LeftPanel();
  main_layout->addWidget(this->left_panel_);

  this->stacked_layout_ = new QStackedLayout();
  main_layout->addLayout(this->stacked_layout_);
}

void MainWindow::initSignals() {
}

}  // namespace hebo