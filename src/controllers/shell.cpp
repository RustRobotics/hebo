// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/shell.h"

#include <ConsoleAppender.h>
#include <Logger.h>
#include <QApplication>
#include <QIcon>
#include <QSharedPointer>

#include "config/config.h"
#include "controllers/main_controller.h"
#include "resources/images/images.h"

namespace hebo {

int runShell(int argc, char** argv) {
  QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QGuiApplication::setApplicationDisplayName("Hebo");
  QGuiApplication::setApplicationName("Hebo");
  QGuiApplication::setApplicationVersion(kAppVersion);
  QGuiApplication::setDesktopFileName("hebo");
  QGuiApplication::setOrganizationDomain("biofan.org");
  QGuiApplication::setOrganizationName("Hebo");
  QGuiApplication::setWindowIcon(QIcon(kImageHebo));

  QApplication application(argc, argv);
  cuteLogger->registerAppender(new ConsoleAppender());
  auto controller = QSharedPointer<MainController>::create();
  controller->showMainWindow();

  return QApplication::exec();
}

}  // namespace hebo
