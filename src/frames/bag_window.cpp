// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/bag_window.h"

namespace hebo {

BagWindow::BagWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
}

void BagWindow::initUi() {
  this->setWindowTitle(tr("Bag"));
}

}  // namespace hebo