// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
#define HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_

#include <QAbstractListModel>

namespace hebo {

class ConnectionModel : public QAbstractListModel {
  Q_OBJECT
 public:
  explicit ConnectionModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
