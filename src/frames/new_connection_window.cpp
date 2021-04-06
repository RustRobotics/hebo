// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/new_connection_window.h"

namespace hebo {

NewConnectionWindow::NewConnectionWindow(QWidget* parent) : QScrollArea(parent) {
  this->initUi();
}

void NewConnectionWindow::initUi() {
  this->setWindowTitle(tr("New Connection"));
  this->form_ = new ConnectionForm(this);
  this->setWidget(this->form_);
}

}  // namespace hebo