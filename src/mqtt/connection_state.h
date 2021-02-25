// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
#define HEBOUI_SRC_MQTT_CONNECTION_STATE_H_

#include <QDebug>
#include <QObject>
#include <QAbstractListModel>

namespace hebo {

class TestClass : public QAbstractListModel {
  Q_OBJECT
  Q_PROPERTY(int indexValue READ indexValue);

 public:
  explicit TestClass(QObject* parent);

  int rowCount(const QModelIndex& parent) const override;

  QVariant data(const QModelIndex& index, int role) const override;

  QHash<int, QByteArray> roleNames() const override;

  [[nodiscard]] int indexValue() const { return this->index_; }

 private:
  int index_{42};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
