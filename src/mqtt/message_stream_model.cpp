// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/message_stream_model.h"

namespace hebo {

MessageStreamModel::MessageStreamModel(QObject* parent)
  : QAbstractListModel(parent) {
  qRegisterMetaType<MqttMessage>("MqttMessage");
  qRegisterMetaType<MqttMessages>("MqttMessages");
}

int MessageStreamModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->messages_.length();
}

QVariant MessageStreamModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }
  const auto& msg = this->messages_.at(index.row());
  switch (role) {
    case kTopicRole: { return msg.topic; }
    case kTopicLengthRole: { return msg.topic.length(); }
    case kQoSRole: { return static_cast<int>(msg.qos); }
    case kIsPublishRole: { return msg.is_publish; }
    case kTimestampRole: { return msg.timestamp; }
    case kPayloadRole: { return msg.payload; }
    case kPayloadLengthRole: { return msg.payload.length(); }
    default: { return {}; }
  }
}

void MessageStreamModel::addMessage(const MqttMessage& message) {
  const int begin = this->messages_.length();
  const int end = begin;
  this->beginInsertRows(QModelIndex(), begin, end);
  this->messages_.append(message);
  this->endInsertRows();
}

void MessageStreamModel::addMessages(const MqttMessages& messages) {
  const int begin = this->messages_.length();
  const int end = begin + messages.length();
  this->beginInsertRows(QModelIndex(), begin, end);
  this->messages_.append(messages);
  this->endInsertRows();
}

}  // namespace hebo
