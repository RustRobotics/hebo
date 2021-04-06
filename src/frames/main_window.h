// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_MAIN_WINDOW_H_
#define HEBO_SRC_FRAMES_MAIN_WINDOW_H_

#include <QWidget>
#include <QStackedLayout>

#include "frames/about_window.h"
#include "frames/bag_window.h"
#include "frames/benchmark_window.h"
#include "frames/connections_window.h"
#include "frames/left_panel.h"
#include "frames/log_window.h"
#include "frames/new_connection_window.h"
#include "frames/settings_window.h"
#include "mqtt/connections_model.h"

namespace hebo {

class MainWindow : public QWidget {
  Q_OBJECT
 public:
  explicit MainWindow(QWidget* parent = nullptr);

  [[nodiscard]] SettingsWindow* settingsWindow() const { return this->settings_window_; }

  void setConnectionsModel(ConnectionsModel* model);

 private slots:
  void switchWindowBydId(LeftPanel::ButtonId id);

 private:
  void initUi();
  void initSignals();

  LeftPanel* left_panel_{nullptr};
  QStackedLayout* stacked_layout_{nullptr};
  AboutWindow* about_window_{nullptr};
  BagWindow* bag_window_{nullptr};
  BenchmarkWindow* benchmark_window_{nullptr};
  ConnectionsWindow* connections_window_{nullptr};
  LogWindow* log_window_{nullptr};
  NewConnectionWindow* new_connection_window_{nullptr};
  SettingsWindow* settings_window_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_MAIN_WINDOW_H_
