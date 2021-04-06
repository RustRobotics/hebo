// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_NEW_CONNECTION_WINDOW_H_
#define HEBO_SRC_FRAMES_NEW_CONNECTION_WINDOW_H_

#include <QScrollArea>

#include "frames/internal/connection_form.h"

namespace hebo {

class NewConnectionWindow : public QScrollArea {
  Q_OBJECT
 public:
  explicit NewConnectionWindow(QWidget* parent = nullptr);

 private:
  void initUi();

  ConnectionForm* form_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_NEW_CONNECTION_WINDOW_H_