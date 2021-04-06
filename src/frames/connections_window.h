// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
#define HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_

#include <QSplitter>

#include "frames/client_frame.h"
#include "frames/internal/connections_list_view.h"

namespace hebo {

class ConnectionsWindow : public QSplitter {
  Q_OBJECT
 public:
  explicit ConnectionsWindow(QWidget* parent = nullptr);

 private:
  void initUi();

  ConnectionsListView* connections_list_view_{nullptr};
  ClientFrame* client_frame_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CONNECTIONS_WINDOW_H_
