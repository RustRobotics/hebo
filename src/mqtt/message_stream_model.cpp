// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/message_stream_model.h"

namespace hebo {
namespace {

constexpr const char* kTopic = "topic";
constexpr const char* kQoS = "qos";
constexpr const char* kIsPublish = "isPublish";
constexpr const char* kTimestamp = "timestamp";
constexpr const char* kPayload = "payload";

}  // namespace

MessageStreamModel::MessageStreamModel(QObject* parent) : QAbstractListModel(parent) {
  qRegisterMetaType<MqttMessage>("MqttMessage");
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

QHash<int, QByteArray> MessageStreamModel::roleNames() const {
  return {
      {kTopicRole, kTopic},
      {kQoSRole, kQoS},
      {kIsPublishRole, kIsPublish},
      {kTimestampRole, kTimestamp},
      {kPayloadRole, kPayload},
  };
}

void MessageStreamModel::addMessage(const MqttMessage& message) {
  const int pos = this->messages_.length();
  this->beginInsertRows(QModelIndex(), pos, pos);
  this->messages_.append(message);
  this->endInsertRows();
}

}  // namespace hebo
