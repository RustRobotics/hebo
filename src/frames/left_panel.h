// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_FRAMES_LEFT_PANEL_H_
#define HEBOUI_SRC_FRAMES_LEFT_PANEL_H_

#include <QWidget>

namespace hebo {

class LeftPanel : public QWidget {
  Q_OBJECT
 public:
  explicit LeftPanel(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBOUI_SRC_FRAMES_LEFT_PANEL_H_
