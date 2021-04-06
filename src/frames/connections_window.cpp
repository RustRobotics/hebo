// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/connections_window.h"


namespace hebo {

ConnectionsWindow::ConnectionsWindow(QWidget* parent) : QSplitter(parent) {
  this->initUi();
}

void ConnectionsWindow::initUi() {
  this->setWindowTitle(tr("Connections"));

  this->connections_list_view_ = new ConnectionsListView();
  this->addWidget(this->connections_list_view_);

  this->client_frame_ = new ClientFrame();
  this->addWidget(this->client_frame_);
}

}  // namespace hebo