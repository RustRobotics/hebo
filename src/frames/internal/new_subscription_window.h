// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_
#define HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_

#include <QFrame>

namespace hebo {

class NewSubscriptionWindow : public QFrame {
  Q_OBJECT
 public:
  explicit NewSubscriptionWindow(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_
