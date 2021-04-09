// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/connections_window.h"

#include <QHBoxLayout>

#include "frames/delegates/connections_delegate.h"

namespace hebo {

ConnectionsWindow::ConnectionsWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
}

void ConnectionsWindow::initUi() {
  this->setWindowTitle(tr("Connections"));
  auto* main_layout = new QHBoxLayout();
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(0);
  this->setLayout(main_layout);

  this->connections_list_view_ = new QListView();
  this->connections_list_view_->setFixedWidth(260);
  this->connections_list_view_->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
  main_layout->addWidget(this->connections_list_view_);
//  auto* delegate = new ConnectionsDelegate(this);
//  this->connections_list_view_->setItemDelegate(delegate);

  this->stacked_widget_ = new QStackedWidget();
  main_layout->addWidget(this->stacked_widget_);
}

void ConnectionsWindow::setConnectionsModel(ConnectionsModel* model) {
  this->connections_list_view_->setModel(model);
  this->model_ = model;
}

void ConnectionsWindow::connectClient(const QString& client_id) {
  this->showClientById(client_id);
}

void ConnectionsWindow::showClientById(const QString& client_id) {
  if (!this->clients_.contains(client_id)) {
    auto* client = this->model_->client(client_id);
    Q_ASSERT(client != nullptr);
    auto* client_frame = new ClientFrame(client_id, client);
    client_frame->show();
    this->clients_.insert(client_id, client_frame);
    this->stacked_widget_->addWidget(client_frame);
  }

  auto* target_frame = this->clients_.value(client_id);
  Q_ASSERT(target_frame != nullptr);
  this->stacked_widget_->setCurrentWidget(target_frame);
}

void ConnectionsWindow::initSignals() {
  connect(this->connections_list_view_, &QListView::clicked,
          this, &ConnectionsWindow::onConnectionsClicked);
}

void ConnectionsWindow::onConnectionsClicked(const QModelIndex& index) {
  const QString client_id = index.data(ConnectionsModel::kIdRole).toString();
  Q_ASSERT(!client_id.isEmpty());
  this->showClientById(client_id);
}

}  // namespace hebo