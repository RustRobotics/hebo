// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
#define HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_

#include <QAbstractListModel>

#include "mqtt/connection_info.h"

namespace hebo {

class ConnectionModel : public QAbstractListModel {
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

  explicit ConnectionModel(QObject* parent = nullptr);


  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

  [[nodiscard]] const ConnectionInfoList& list() const { return this->list_; }

  bool getConnectionInfo(const QString& name, ConnectionInfo& info) const;

  Q_INVOKABLE QVariantMap row(int index) const;

  bool deleteConnectionInfo(const QString& name);

 public slots:
  void addConnectionInfo(const ConnectionInfo& info);

  void setList(const ConnectionInfoList& list);

 private:
  ConnectionInfoList list_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_MODEL_H_
