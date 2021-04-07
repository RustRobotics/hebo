// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/client_frame.h"

#include <QHBoxLayout>
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
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(0);
  this->setLayout(main_layout);

  this->tool_bar_ = new QFrame();
  this->tool_bar_->setFixedHeight(48);
  auto* tool_bar_layout = new QHBoxLayout();
  this->tool_bar_->setLayout(tool_bar_layout);

  auto* bottom_layout = new QHBoxLayout();
  main_layout->addLayout(bottom_layout);

  this->subscriptions_list_view_ = new QListView();
  this->subscriptions_list_view_->setFixedWidth(230);
  bottom_layout->addWidget(this->subscriptions_list_view_);

  auto* messages_layout = new QVBoxLayout();
  bottom_layout->addLayout(messages_layout);

  this->messages_edit_ = new QPlainTextEdit();
  messages_layout->addWidget(this->messages_edit_);

  this->topic_edit_ = new QLineEdit();
  messages_layout->addWidget(this->topic_edit_);

  this->payload_edit_ = new QTextEdit();
  messages_layout->addWidget(this->payload_edit_);
}

void ClientFrame::initSignals() {

}

}  // namespace hebo
