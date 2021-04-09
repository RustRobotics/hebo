// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
#define HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_

#include <QListView>
#include <QFrame>
#include <QStackedWidget>

#include "frames/client_frame.h"
#include "mqtt/connections_model.h"

namespace hebo {

class ConnectionsWindow : public QFrame {
  Q_OBJECT
 public:
  explicit ConnectionsWindow(QWidget* parent = nullptr);

  // This class does not take ownership of model.
  void setConnectionsModel(ConnectionsModel* model);

 public slots:
  void connectClient(const QString& client_id);

  void showClientById(const QString& client_id);

 private slots:
  void onConnectionsClicked(const QModelIndex& index);

 private:
  void initUi();
  void initSignals();

  QListView* connections_list_view_{nullptr};
  QStackedWidget* stacked_widget_{nullptr};
  ConnectionsModel* model_{nullptr};

  // Map of ClientID => ClientFrame
  QMap<QString, ClientFrame*> clients_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
