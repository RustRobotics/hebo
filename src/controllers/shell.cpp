// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/shell.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>

#include "ui/ui.h"

namespace hebo {

int runShell(int argc, char** argv) {
  QGuiApplication application(argc, argv);
  QQmlApplicationEngine engine(kUiMainWindow);

  return QGuiApplication::exec();
}

}  // namespace hebo