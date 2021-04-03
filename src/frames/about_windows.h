// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_FRAMES_ABOUT_WINDOWS_H_
#define HEBOUI_SRC_FRAMES_ABOUT_WINDOWS_H_

#include <QWidget>

namespace hebo {

class AboutWindows : public QWidget {
  Q_OBJECT
 public:
  explicit AboutWindows(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBOUI_SRC_FRAMES_ABOUT_WINDOWS_H_
