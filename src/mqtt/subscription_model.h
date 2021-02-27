// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_SUBSCRIPTION_MODEL_H_
#define HEBOUI_SRC_MQTT_SUBSCRIPTION_MODEL_H_

#include <QAbstractListModel>
#include <QColor>

#include "mqtt/connect_config.h"

namespace hebo {

struct Subscription {
  QString topic{};
  QColor color{};
  QoS qos{};
};
using SubscriptionList = QVector<Subscription>;

class SubscriptionModel : public QAbstractListModel {
  Q_OBJECT
 public:
  enum SubscriptionRole : int {
    kTopicRole = Qt::UserRole + 1,
    kColorRole,
    kQoSRole,
  };
  Q_ENUM(SubscriptionRole);

  explicit SubscriptionModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

  bool hasSubscription(const QString& topic);

  bool addSubscription(const QString& topic, int qos, const QColor& color);

  bool removeSubscription(const QString& topic);

 private:
  SubscriptionList list_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_SUBSCRIPTION_MODEL_H_
