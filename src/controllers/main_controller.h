// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
#define HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_

#include <QObject>
#include <QQmlApplicationEngine>
#include <QThread>

#include "controllers/log_manager.h"
#include "controllers/settings_manager.h"
#include "controllers/update_manager.h"

namespace hebo {

class MainController : public QObject {
  Q_OBJECT
 public:
  explicit MainController(QObject* parent = nullptr);
  ~MainController() override;

  void showMainWindow();

 private:
  QQmlApplicationEngine* engine_{};

  QThread* updater_thread_;

  LogManager* log_manager_;
  UpdateManager* update_manager_;
  SettingsManager* settings_manager_;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
