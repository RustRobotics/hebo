// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
#define HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_

#include <QAbstractListModel>

#include "mqtt/connection_info.h"

namespace hebo {

enum ConnectionRole : int {
  kConnectionNameRole = Qt::UserRole + 1,
  kConnectionClientIdRole,
  kConnectionProtocolRole,
  kConnectionHostRole,
  kConnectionPortRole,
  kConnectionQoSRole,
  kConnectionUsernameRole,
  kConnectionPasswordRole,
  kConnectionTlsRole,
  kConnectionCleanSessionRole,
  kConnectionDescriptionRole,
};

class ConnectionModel : public QAbstractListModel {
  Q_OBJECT
 public:
  explicit ConnectionModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

  void addConnectionInfo(const ConnectionInfo& info);

 private:
  ConnectionInfoList list_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
