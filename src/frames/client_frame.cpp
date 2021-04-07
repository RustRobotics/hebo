// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/client_frame.h"

#include <QHBoxLayout>
#include <QVBoxLayout>

#include "resources/fonts/fonts.h"

namespace hebo {

ClientFrame::ClientFrame(const QString& client_id, MqttClient* client, QWidget* parent)
    : QFrame(parent),
      client_id_(client_id),
      client_(client) {
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
  tool_bar_layout->setContentsMargins(0, 0, 0, 0);
  tool_bar_layout->setSpacing(12);
  this->tool_bar_->setLayout(tool_bar_layout);
  main_layout->addWidget(this->tool_bar_);

  this->title_label_ = new QLabel();
  this->title_label_->setText(this->client_->config().name);
  tool_bar_layout->addWidget(this->title_label_);

  tool_bar_layout->addStretch();
  this->connect_button_ = new FontIconButton(kFontElIconCaretRight);
  tool_bar_layout->addWidget(this->connect_button_);

  this->disconnect_button_ = new FontIconButton(kFontElIconSwitchButton);
  tool_bar_layout->addWidget(this->disconnect_button_);
  this->disconnect_button_->hide();

  this->edit_button_ = new FontIconButton(kFontElIconEditOutline);
  tool_bar_layout->addWidget(this->edit_button_);

  this->options_button_ = new FontIconButton(kFontElIconMoreOutline);
  tool_bar_layout->addWidget(this->options_button_);

  auto* bottom_layout = new QHBoxLayout();
  bottom_layout->setContentsMargins(0, 0, 0, 0);
  bottom_layout->setSpacing(0);
  main_layout->addLayout(bottom_layout);

  this->subscriptions_list_view_ = new QListView();
  this->subscriptions_list_view_->setFixedWidth(230);
  bottom_layout->addWidget(this->subscriptions_list_view_);
  this->subscriptions_list_view_->setModel(this->client_->subscriptions());

  auto* messages_layout = new QVBoxLayout();
  messages_layout->setContentsMargins(0, 0, 0, 0);
  messages_layout->setSpacing(0);
  bottom_layout->addLayout(messages_layout);

  this->messages_edit_ = new QPlainTextEdit();
  messages_layout->addWidget(this->messages_edit_);

  this->topic_edit_ = new QLineEdit();
  messages_layout->addWidget(this->topic_edit_);

  this->payload_edit_ = new QTextEdit();
  messages_layout->addWidget(this->payload_edit_);
}

void ClientFrame::initSignals() {
  Q_ASSERT(this->client_ != nullptr);
  connect(this->connect_button_, &FontIconButton::clicked,
          this->client_, &MqttClient::requestConnect);
  connect(this->disconnect_button_, &FontIconButton::clicked,
          this->client_, &MqttClient::requestDisconnect);
  connect(this->client_, &MqttClient::stateChanged,
          this, &ClientFrame::onClientStateChanged);
}

void ClientFrame::onClientStateChanged(ConnectionState state) {
  switch (state) {
    case ConnectionState::ConnectionConnecting:  // fall through
    case ConnectionState::ConnectionConnected: {
      this->connect_button_->hide();
      this->disconnect_button_->show();
      break;
    }
    case ConnectionState::ConnectionConnectFailed:  // fall through
    case ConnectionState::ConnectionDisconnecting:  // fall through
    case ConnectionState::ConnectionDisconnected: {
      this->connect_button_->show();
      this->disconnect_button_->hide();
      break;
    }
    default: {
      Q_UNREACHABLE();
      break;
    }
  }
}

}  // namespace hebo
