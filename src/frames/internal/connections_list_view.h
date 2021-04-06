// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_CONNECTIONS_LIST_VIEW_H_
#define HEBO_SRC_FRAMES_INTERNAL_CONNECTIONS_LIST_VIEW_H_

#include <QListView>

namespace hebo {

class ConnectionsListView : public QListView {
  Q_OBJECT
 public:
  explicit ConnectionsListView(QWidget* parent = nullptr);

 private:
  void initUi();
  void initSignals();
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_CONNECTIONS_LIST_VIEW_H_
