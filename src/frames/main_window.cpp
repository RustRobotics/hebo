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

  this->messages_window_ = new MessagesWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kMessages, this->messages_window_);

  this->benchmark_window_ = new BenchmarkWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kBenchmark, this->benchmark_window_);

  this->bag_window_ = new BagWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kBag, this->bag_window_);

  this->log_window_ = new LogWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kLog, this->log_window_);

  this->about_window_ = new AboutWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kAbout, this->about_window_);

  this->settings_window_ = new SettingsWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kSettings, this->settings_window_);
}

void MainWindow::initSignals() {
  connect(this->left_panel_, &LeftPanel::activeChanged, [=](LeftPanel::ButtonId id) {
    this->stacked_layout_->setCurrentIndex(id);
    auto* widget = this->stacked_layout_->widget(id);
    if (widget != nullptr) {
      this->setWindowTitle(widget->windowTitle());
    } else {
      qCritical() << "widget is null, id:" << id;
    }
  });
}

}  // namespace hebo