// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/main_controller.h"

#include <QDebug>
#include <QDir>
#include <QFontDatabase>
#include <QGuiApplication>
#include <QLibraryInfo>
#include <QTranslator>

#include "frames/main_window.h"
#include "frames/settings_window.h"
#include "resources/fonts/fonts.h"

namespace hebo {
namespace {

constexpr const char* kI18Template = ":/i18n/hebo-%1.qm";

}  // namespace

MainController::MainController(QObject* parent)
    : QObject(parent),
      settings_manager_(new SettingsManager(this)),
      update_manager_(new UpdateManager()),
      update_thread_(new QThread()) {
  this->update_manager_->moveToThread(this->update_thread_);
  loadExternalFonts();
  this->installTranslators();
  this->initSignals();
  this->update_thread_->start();
}

MainController::~MainController() {
  this->update_thread_->exit();
  this->update_thread_->deleteLater();
}

void MainController::showMainWindow() {
  auto* main_window = new MainWindow();
  this->initWindow(main_window);
  main_window->resize(1020, 720);
  main_window->show();
}

void MainController::initWindow(MainWindow* window) {
  connect(window, &MainWindow::destroyed,
          window, &MainWindow::deleteLater);

  auto* settings_window = window->settingsWindow();
  Q_ASSERT(settings_window != nullptr);
  connect(this->settings_manager_, &SettingsManager::localeChanged,
          settings_window, &SettingsWindow::setLocale);
  connect(this->settings_manager_, &SettingsManager::retryConnectionsChanged,
          settings_window, &SettingsWindow::setRetryConnection);
  connect(this->settings_manager_, &SettingsManager::autoUpdateChanged,
          settings_window, &SettingsWindow::setAutoUpdate);
  connect(this->settings_manager_, &SettingsManager::themeChanged,
          settings_window, &SettingsWindow::setTheme);
  settings_window->setLocale(this->settings_manager_->locale());
  settings_window->setRetryConnection(this->settings_manager_->retryConnections());
  settings_window->setAutoUpdate(this->settings_manager_->autoUpdate());
  settings_window->setTheme(this->settings_manager_->theme());
}

void MainController::installTranslators() {
  auto* local_translator = new QTranslator(this);
  const QString file = QString(kI18Template).arg(QLocale().name());
  if (local_translator->load(file)) {
    QGuiApplication::installTranslator(local_translator);
  } else {
    qWarning() << "Failed to load translator file:" << file;
  }
}

void MainController::initSignals() {
  connect(this->update_manager_, &UpdateManager::destroyed,
          this->update_manager_, &UpdateManager::deleteLater);
  connect(this->update_thread_, &QThread::finished,
          this->update_thread_, &QThread::deleteLater);
}

void loadExternalFonts() {
  for (const char* font : kExternalFonts) {
    QFontDatabase::addApplicationFont(font);
  }
}

}  // namespace hebo
