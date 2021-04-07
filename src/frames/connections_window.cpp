// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/connections_window.h"

namespace hebo {

ConnectionsWindow::ConnectionsWindow(QWidget* parent) : QSplitter(parent) {
  this->initUi();
  this->initSignals();
}

void ConnectionsWindow::initUi() {
  this->setWindowTitle(tr("Connections"));

  this->connections_list_view_ = new ConnectionsListView();
  this->addWidget(this->connections_list_view_);

  this->stacked_widget_ = new QStackedWidget();
  this->addWidget(this->stacked_widget_);
}

void ConnectionsWindow::setConnectionsModel(ConnectionsModel* model) {
  this->connections_list_view_->setConnectionsModel(model);
}

void ConnectionsWindow::connectClient(const QString& client_id) {
  this->showClientById(client_id);
}

void ConnectionsWindow::showClientById(const QString& client_id) {
  if (!this->clients_.contains(client_id)) {
    auto* client_frame = new ClientFrame(client_id);
    client_frame->show();
    this->clients_.insert(client_id, client_frame);
    this->stacked_widget_->addWidget(client_frame);
  }

  auto* target_frame = this->clients_.value(client_id);
  Q_ASSERT(target_frame != nullptr);
  this->stacked_widget_->setCurrentWidget(target_frame);
}

void ConnectionsWindow::initSignals() {
  connect(this->connections_list_view_, &ConnectionsListView::rowClicked,
          this, &ConnectionsWindow::showClientById);
}

}  // namespace hebo