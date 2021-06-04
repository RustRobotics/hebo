// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/benchmark_window.h"

namespace hebo {

BenchmarkWindow::BenchmarkWindow(QWidget* parent) : QWidget(parent) {
  this->initUi();
}

void BenchmarkWindow::initUi() {
  this->setWindowTitle(tr("Benchmark"));
}

}  // namespace hebo