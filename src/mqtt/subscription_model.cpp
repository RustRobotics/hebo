// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/subscription_model.h"

#include "base/color.h"

namespace hebo {
namespace {

constexpr const char* kTopic = "topic";
constexpr const char* kColor = "color";
constexpr const char* kQoS = "qos";

}  // namespace

SubscriptionModel::SubscriptionModel(QObject* parent) : QAbstractListModel(parent) {
  this->list_.append(Subscription{"Hello", Qt::red, AtMostOnce});
}

int SubscriptionModel::rowCount(const QModelIndex& parent) const {
  qDebug() << __func__ << this->list_.length();
  Q_UNUSED(parent);
  return this->list_.length();
}

QVariant SubscriptionModel::data(const QModelIndex& index, int role) const {
  if (index.isValid()) {
    return {};
  }
  const auto& sub = this->list_.at(index.row());
  switch (role) {
    case kTopicRole: {
      return sub.topic;
    }
    case kColorRole: {
      return sub.color;
    }
    case kQoSRole: {
      return sub.qos;
    }
    default: {
      return {};
    }
  }
}

QHash<int, QByteArray> SubscriptionModel::roleNames() const {
  return {
      {kTopicRole, kTopic},
      {kColorRole, kColor},
      {kQoSRole, kQoS},
  };
}

bool SubscriptionModel::hasSubscription(const QString& topic) {
  for (const auto& sub : this->list_) {
    if (sub.topic == topic) {
      return true;
    }
  }
  return false;
}

bool SubscriptionModel::addSubscription(const QString& topic, int qos, const QString& color) {
  if (this->hasSubscription(topic)) {
    return false;
  }
  this->beginResetModel();
  this->list_.append({topic, parseColor(color), static_cast<QoS>(qos)});
  this->endResetModel();
  return true;
}

bool SubscriptionModel::removeSubscription(const QString& topic) {
  for (int index = 0; index < this->list_.length(); ++index) {
    if (this->list_.at(index).topic == topic) {
      this->beginResetModel();
      this->list_.removeAt(index);
      this->endResetModel();
      return true;
    }
  }
  return false;
}

}  // namespace hebo