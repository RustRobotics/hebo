// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_FRAMES_MAIN_WINDOW_H_
#define HEBOUI_SRC_FRAMES_MAIN_WINDOW_H_

#include <QMainWindow>
#include <QStackedWidget>

#include "frames/left_panel.h"
#include "frames/log_window.h";

namespace hebo {

class MainWindow : public QMainWindow {
  Q_OBJECT
 public:
  explicit MainWindow(QWidget* parent = nullptr);

 private:
  void initUi();
  void initSignals();
  void initMenu();

  LeftPanel* left_panel_{nullptr};
  QStackedWidget* stacked_widget_{nullptr};

};

}  // namespace hebo

#endif  // HEBOUI_SRC_FRAMES_MAIN_WINDOW_H_
