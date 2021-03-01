// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/main_controller.h"

#include <QTranslator>
#include <QDir>
#include <QGuiApplication>
#include <QLibraryInfo>
#include <QQmlContext>

#include "ui/ui.h"

namespace hebo {

MainController::MainController(QObject* parent)
    : QObject(parent),
      engine_(new QQmlApplicationEngine(this)),
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
  auto* context = this->engine_->rootContext();
  context->setContextProperty("logManager", this->log_manager_);
  context->setContextProperty("updateManager", this->update_manager_);
  context->setContextProperty("settingsManager", this->settings_manager_);
  context->setContextProperty("connectManager", this->connect_manager_);

  this->engine_->load(kUiMainWindow);
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
