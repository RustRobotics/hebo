// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/client_frame.h"

#include <QHBoxLayout>
#include <QResizeEvent>
#include <QVBoxLayout>

#include "frames/delegates/messages_delegate.h"
#include "resources/fonts/fonts.h"

namespace hebo {

ClientFrame::ClientFrame(const QString& client_id, MqttClient* client, QWidget* parent)
    : QFrame(parent),
      client_id_(client_id),
      client_(client) {
  Q_ASSERT(this->client_ != nullptr);
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

  this->messages_list_view_ = new QListView();
  this->messages_list_view_->setSpacing(12);
  auto* messages_delegate = new MessagesDelegate(this);
  this->messages_list_view_->setItemDelegate(messages_delegate);
  this->messages_list_view_->setModel(this->client_->messages());
  messages_layout->addWidget(this->messages_list_view_);

  this->topic_edit_ = new QLineEdit();
  this->topic_edit_->setPlaceholderText(tr("Topic"));
  messages_layout->addWidget(this->topic_edit_);

  this->payload_edit_ = new QPlainTextEdit();
  this->payload_edit_->setFixedHeight(110);
  this->payload_edit_->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
  this->payload_edit_->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
  messages_layout->addWidget(this->payload_edit_);

  this->publish_button_ = new FontIconButton(kFontElIconPosition, this);
  this->publish_button_->setFixedSize(20, 20);
  this->publish_button_->show();
}

void ClientFrame::initSignals() {
  Q_ASSERT(this->client_ != nullptr);
  connect(this->connect_button_, &FontIconButton::clicked,
          this->client_, &MqttClient::requestConnect);
  connect(this->disconnect_button_, &FontIconButton::clicked,
          this->client_, &MqttClient::requestDisconnect);
  connect(this->client_, &MqttClient::stateChanged,
          this, &ClientFrame::onClientStateChanged);
  connect(this->publish_button_, &FontIconButton::clicked,
          this, &ClientFrame::onPublishButtonClicked);
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

void ClientFrame::resizeEvent(QResizeEvent* event) {
  QWidget::resizeEvent(event);
  this->publish_button_->move(event->size().width() - 36,
                              event->size().height() - 36);
}

void ClientFrame::onPublishButtonClicked() {
  if (this->client_->state() != ConnectionState::ConnectionConnected) {
    return;
  }
  const QString topic = this->topic_edit_->text();
  if (topic.isEmpty()) {
    return;
  }
  const QString payload = this->payload_edit_->toPlainText();
  const QoS qos = QoS::AtMostOnce;
  const bool retain = false;
  this->client_->requestPublish(topic, payload.toUtf8(), qos, retain);
}

}  // namespace hebo
