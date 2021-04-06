// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
#define HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_

#include <QSplitter>

#include "frames/client_frame.h"
#include "frames/internal/connections_list_view.h"
#include "mqtt/connections_model.h"

namespace hebo {

class ConnectionsWindow : public QSplitter {
  Q_OBJECT
 public:
  explicit ConnectionsWindow(QWidget* parent = nullptr);

  // This class does not take ownership of model.
  void setConnectionsModel(ConnectionsModel* model);

 private:
  void initUi();

  ConnectionsListView* connections_list_view_{nullptr};
  ClientFrame* client_frame_{nullptr};
  ConnectionsModel* model_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
