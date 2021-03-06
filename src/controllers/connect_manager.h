// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_

#include <QAbstractListModel>

#include "mqtt/connect_config.h"
#include "mqtt/mqtt_client.h"

namespace hebo {

class ConnectManager : public QAbstractListModel {
  Q_OBJECT

 public:
  enum ConnectionRole : int32_t {
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
    kConnectionStateRole,
  };
  Q_ENUM(ConnectionRole);

  explicit ConnectManager(QObject* parent=nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

  Q_INVOKABLE [[nodiscard]] QString configId(int index) const;

  Q_INVOKABLE [[nodiscard]] QVariantMap config(const QString& config_id) const;

  Q_INVOKABLE hebo::MqttClient* client(const QString& config_id);

  Q_INVOKABLE [[nodiscard]] QString newClientId() const;

 public slots:
  // Connections management
  // Protocol V3.1.1
  void addConnection(const QString& name,
                     const QString& client_id,
                     const QString& protocol,
                     const QString& host,
                     int port,
                     QoS qos,
                     bool clean_session);

 signals:

 private:
  void loadConnInfo();
  void saveConnInfo();

  QString conn_file_;
  QVector<ConnectConfig> configs_{};

  // ConfigId -> Client*
  QMap<QString, MqttClient*> clients_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
