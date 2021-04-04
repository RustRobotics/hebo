// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/about_windows.h"

namespace hebo {

AboutWindows::AboutWindows(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
}

void AboutWindows::initUi() {
  this->setWindowTitle(tr("About"));
}

void AboutWindows::initSignals() {

}

}  // namespace hebo