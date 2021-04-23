// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
#define HEBO_SRC_CONTROLLERS_MAIN_CONTROLLER_H_

#include <QObject>
#include <QThread>

//#include "controllers/log_manager.h"
#include "controllers/settings_manager.h"
#include "controllers/update_manager.h"
#include "formats/theme.h"
#include "mqtt/connections_model.h"

namespace hebo {

class MainWindow;

class MainController : public QObject {
  Q_OBJECT
 public:
  explicit MainController(QObject* parent = nullptr);
  ~MainController() override;

  QString theme() const;

 public slots:
  void showMainWindow();

 private:
  void onThemeChanged(ThemeType theme);

 private:
  void initSignals();
  void installTranslators();
  void initWindow(MainWindow* window);

  ConnectionsModel* connections_model_;
//  LogManager* log_manager_;
  SettingsManager* settings_manager_;
  UpdateManager* update_manager_;
  QThread* update_thread_;
};

void loadExternalFonts();

}  // namespace hebo

#endif  // HEBO_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
