// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/client_frame.h"

#include <QLabel>
#include <QVBoxLayout>

namespace hebo {

ClientFrame::ClientFrame(const QString& client_id, QWidget* parent)
    : QFrame(parent),
      client_id_(client_id) {
  this->initUi();
  this->initSignals();
}

void ClientFrame::initUi() {
  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  main_layout->addWidget(new QLabel(this->client_id_));
}

void ClientFrame::initSignals() {

}

}  // namespace hebo