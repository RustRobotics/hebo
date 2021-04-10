// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_MQTT_MESSAGE_STREAM_MODEL_H_
#define HEBO_SRC_MQTT_MESSAGE_STREAM_MODEL_H_

#include <QAbstractListModel>
#include <QSharedPointer>
#include <QVector>

#include "formats/connect_config.h"

namespace hebo {

struct MqttMessage {
  QString topic{};
  QoS qos{};
  bool is_publish{false};
  QDateTime timestamp{QDateTime::currentDateTime()};
  QByteArray payload{};
};

using MqttMessages = QVector<MqttMessage>;
using MqttMessagesPtr = QSharedPointer<MqttMessages>;

class MessageStreamModel : public QAbstractListModel {
  Q_OBJECT
 public:
  enum MessageRole : int32_t {
    kTopicRole = Qt::UserRole + 1,
    kTopicLengthRole,
    kQoSRole,
    kIsPublishRole,
    kTimestampRole,
    kPayloadRole,
    kPayloadLengthRole,
  };
  Q_ENUM(MessageRole);

  explicit MessageStreamModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  bool row(int row, MqttMessage& msg) const {
    if (row >= 0 && row < this->messages_.length()) {
      msg = this->messages_.at(row);
      return true;
    }
    return false;
  }

 public slots:
  void addMessage(const MqttMessage& message);

  void addMessages(const MqttMessages& messages);

 private:
  MqttMessages messages_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_MQTT_MESSAGE_STREAM_MODEL_H_
