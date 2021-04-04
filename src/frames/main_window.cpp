// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/main_window.h"

namespace hebo {

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent) {
  this->initUi();
  this->initMenu();
  this->initSignals();
}

void MainWindow::initUi() {
  this->left_panel_ = new LeftPanel();

  this->stacked_widget_ = new QStackedWidget(this);
  this->setCentralWidget(this->stacked_widget_);
}

void MainWindow::initSignals() {

}

void MainWindow::initMenu() {

}
}  // namespace hebo