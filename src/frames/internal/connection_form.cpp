// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/connection_form.h"

#include <QFormLayout>
#include <QLabel>

namespace hebo {

ConnectionForm::ConnectionForm(QWidget* parent) : QFrame(parent) {
  this->initUi();
}

void ConnectionForm::initUi() {
  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  this->initGeneralForm(main_layout);
  this->initAdvancedForm(main_layout);
}

void ConnectionForm::initGeneralForm(QVBoxLayout* main_layout) {
  auto* title_label = new QLabel(tr("General"));
  main_layout->addWidget(title_label);

  auto* layout = new QFormLayout();
  main_layout->addLayout(layout);

  this->name_edit_ = new QLineEdit();
  layout->addRow(new QLabel(tr("Name")), this->name_edit_);

  this->client_id_edit_ = new QLineEdit();
  layout->addRow(new QLabel(tr("Client ID")), this->client_id_edit_);

  this->protocol_box_ = new QComboBox();
  this->protocol_model_ = new ProtocolModel(this);
  this->protocol_box_->setModel(this->protocol_model_);
  this->hostname_edit_ = new QLineEdit();
  auto* host_layout = new QHBoxLayout();
  host_layout->addWidget(this->protocol_box_);
  host_layout->addWidget(this->hostname_edit_);
  layout->addRow(new QLabel("Host"), host_layout);

  this->port_box_ = new QSpinBox();
  layout->addRow(new QLabel("Port"), this->port_box_);

  this->username_edit_ = new QLineEdit();
  layout->addRow(new QLabel("Username"), this->username_edit_);

  this->password_edit_ = new QLineEdit();
  layout->addRow(new QLabel("Password"), this->password_edit_);

  this->tls_switch_ = new SwitchButton();
  layout->addRow(new QLabel("SSL/TLS"), this->tls_switch_);
}

void ConnectionForm::initAdvancedForm(QVBoxLayout* main_layout) {
  auto* title_label = new QLabel(tr("Advanced"));
  main_layout->addWidget(title_label);
}


}  // namespace hebo