// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_

#include <QAbstractListModel>

#include "mqtt/connect_config.h"

namespace hebo {

class ConnectManager : public QAbstractListModel {
  Q_OBJECT
 public:
  enum ConnectionRole : int {
    kNameRole = Qt::UserRole + 1,
    kClientIdRole,
    kProtocolRole,
    kHostRole,
    kPortRole,
    kQoSRole,
    kUsernameRole,
    kPasswordRole,
    kTlsRole,
    kCleanSessionRole,
    kDescriptionRole,
  };
  Q_ENUM(ConnectionRole);

  explicit ConnectManager(QObject* parent=nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

 private:
  QVector<ConnectConfig> configs_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
