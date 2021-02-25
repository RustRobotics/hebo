// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_

#include <QObject>
#include <QSharedPointer>

#include "mqtt/connection_info.h"
#include "mqtt/connection_state.h"
#include "mqtt/mqtt_client.h"
#include "mqtt/connection_model.h"

namespace hebo {

class ConnectManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(ConnectionModel* model READ model);

 public:
  explicit ConnectManager(QObject* parent = nullptr);

  ConnectionModel* model() { return this->model_; }
  [[nodiscard]] const ConnectionModel* model() const { return this->model_; }

 public slots:
  // Connections management
  // Protocol V3.1.1
  void addConnection(const QString& name,
                     const QString& client_id,
                     const QString& protocol,
                     const QString& host,
                     int port,
                     int qos,
                     bool clean_session);

  void deleteConnection(const QString& name);

  void requestConnect(const QString& name);

 signals:

 private:
  void loadConnInfo();
  void saveConnInfo();

  QString conn_file_;
  ConnectionModel* model_{nullptr};
  QMap<QString, MqttClientPtr> clients_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_CONNECT_MANAGER_H_
