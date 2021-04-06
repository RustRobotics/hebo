// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/connections_list_view.h"

#include "frames/delegates/connections_delegate.h"

namespace hebo {

ConnectionsListView::ConnectionsListView(QWidget* parent) : QListView(parent) {
  this->initUi();
  this->initSignals();
}

void ConnectionsListView::initUi() {
  auto* delegate = new ConnectionsDelegate(this);
  this->setItemDelegate(delegate);
  this->setSelectionMode(QListView::SelectionMode::SingleSelection);
}

void ConnectionsListView::initSignals() {

}

}  // namespace hebo