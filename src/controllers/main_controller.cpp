// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/main_controller.h"

#include <QQmlContext>

#include "ui/ui.h"

namespace hebo {

MainController::MainController(QObject* parent)
    : QObject(parent),
      engine_(new QQmlApplicationEngine(this)),
      updater_thread_(new QThread()),
      log_manager_(new LogManager(this)),
      update_manager_(new UpdateManager()),
      settings_manager_(new SettingsManager(this)) {

  update_manager_->moveToThread(updater_thread_);
  updater_thread_->start();
}

MainController::~MainController() {
  this->updater_thread_->exit();
  this->updater_thread_->deleteLater();
}

void MainController::showMainWindow() {
  auto* context = this->engine_->rootContext();
  context->setContextProperty("logManager", this->log_manager_);
  context->setContextProperty("updateManager", this->update_manager_);
  context->setContextProperty("settingsManager", this->settings_manager_);

  this->engine_->load(kUiMainWindow);
}

}  // namespace hebo