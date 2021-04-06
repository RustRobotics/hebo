// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CLIENT_FRAME_H_
#define HEBO_SRC_FRAMES_CLIENT_FRAME_H_

#include <QFrame>

namespace hebo {

class ClientFrame : public QFrame {
  Q_OBJECT
 public:
  explicit ClientFrame(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CLIENT_FRAME_H_
