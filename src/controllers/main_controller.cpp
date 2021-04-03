// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/main_controller.h"

#include <QDir>
#include <QGuiApplication>
#include <QLibraryInfo>
#include <QTranslator>

#include "frames/main_window.h"

namespace hebo {

MainController::MainController(QObject* parent)
    : QObject(parent),
      updater_thread_(new QThread()),
      log_manager_(new LogManager(this)),
      update_manager_(new UpdateManager()),
      settings_manager_(new SettingsManager(this)),
      connect_manager_(new ConnectManager(this)) {
  this->installTranslators();
  update_manager_->moveToThread(updater_thread_);
  updater_thread_->start();
}

MainController::~MainController() {
  this->updater_thread_->exit();
  this->updater_thread_->deleteLater();
}

void MainController::showMainWindow() {
  auto* main_window = new MainWindow();
  connect(main_window, &MainWindow::destroyed,
          main_window, &MainWindow::deleteLater);
  main_window->show();
}

void MainController::installTranslators() {
  constexpr const char* kI18Template = ":/i18n/hebo-%1.qm";
  auto* local_translator = new QTranslator(this);
  const QString file = QString(kI18Template).arg(QLocale().name());
  if (local_translator->load(file)) {
    QGuiApplication::installTranslator(local_translator);
  } else {
    qWarning() << "Failed to load translator file:" << file;
  }

}

}  // namespace hebo
