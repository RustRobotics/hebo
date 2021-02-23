// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/shell.h"

#include <QGuiApplication>

#include "controllers/main_controller.h"

namespace hebo {

int runShell(int argc, char** argv) {
  QGuiApplication application(argc, argv);
  QScopedPointer<MainController> controller(new MainController());
  controller->showMainWindow();

  return QGuiApplication::exec();
}

}  // namespace hebo