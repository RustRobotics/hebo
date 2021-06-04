// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/subscription_model.h"

namespace hebo {

SubscriptionModel::SubscriptionModel(QObject* parent) : QAbstractListModel(parent) {
}

int SubscriptionModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->list_.length();
}

QVariant SubscriptionModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }
  const auto& sub = this->list_.at(index.row());
  switch (role) {
    case kTopicRole: {
      return sub.topic;
    }
    case Qt::ToolTipRole:  // fall through
    case Qt::DisplayRole:  // fall through
    case kDescriptionRole: {
      return QString("%1 (qos=%2)").arg(sub.topic).arg(static_cast<int>(sub.qos));
    }
    case Qt::DecorationRole:  // fall through
    case kColorRole: {
      return sub.color;
    }
    case kQoSRole: {
      return static_cast<int>(sub.qos);
    }
    default: { return {}; }
  }
}

bool SubscriptionModel::hasSubscription(const QString& topic) {
  return std::any_of(this->list_.begin(), this->list_.end(), [&](const Subscription& sub) {
    return sub.topic == topic;
  });
}

bool SubscriptionModel::addSubscription(const QString& topic, QoS qos, const QColor& color) {
  if (this->hasSubscription(topic)) {
    return false;
  }
  this->beginResetModel();
  this->list_.append({topic, qos, color});
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