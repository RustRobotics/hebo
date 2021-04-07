// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/new_connection_window.h"

namespace hebo {

NewConnectionWindow::NewConnectionWindow(QWidget* parent) : QScrollArea(parent) {
  this->initUi();
  this->initSignals();
}

void NewConnectionWindow::initUi() {
  this->setWindowTitle(tr("New Connection"));
  this->form_ = new ConnectionForm();
  this->form_->regenerateClientId();
  this->setWidget(this->form_);
  this->setAlignment(Qt::AlignHCenter | Qt::AlignTop);
}

void NewConnectionWindow::initSignals() {
  connect(this->form_, &ConnectionForm::connectRequested,
          this, &NewConnectionWindow::addNewConnection);
}

void NewConnectionWindow::addNewConnection(const ConnectConfig& config) {
  Q_ASSERT(this->model_ != nullptr);
  this->model_->addConnection(config);
  emit this->newConnectionAdded(config.id);
}

}  // namespace hebo