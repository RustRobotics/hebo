// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_MESSAGE_STREAM_MODEL_H_
#define HEBOUI_SRC_MQTT_MESSAGE_STREAM_MODEL_H_

#include <QAbstractListModel>

#include "mqtt/connect_config.h"

namespace hebo {

struct MqttMessage {
  QString topic{};
  QoS qos{};
  bool is_publish{};
  QDateTime timestamp{};
  QByteArray payload{};
};

using MqttMessages = QVector<MqttMessage>;

class MessageStreamModel : public QAbstractListModel {
  Q_OBJECT
 public:
  enum MessageRole : int32_t {
    kTopicRole = Qt::UserRole + 1,
    kQoSRole,
    kIsPublishRole,
    kTimestampRole,
    kPayloadRole,
  };
  Q_ENUM(MessageRole);

  explicit MessageStreamModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

 private:
  MqttMessages messages_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_MESSAGE_STREAM_MODEL_H_
