// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/connections_window.h"

namespace hebo {

ConnectionsWindow::ConnectionsWindow(QWidget* parent) : QWidget(parent) {
  this->initUi();
}

void ConnectionsWindow::initUi() {
  this->setWindowTitle(tr("Connections"));
}

}  // namespace hebo