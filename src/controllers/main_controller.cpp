// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/main_controller.h"

#include <QApplication>
#include <QFontDatabase>
#include <QLibraryInfo>
#include <QTranslator>
#include <rusty/widgets/adwaita_style.h>
#include <rusty/widgets/theme_manager.h>

#include "frames/main_window.h"
#include "frames/settings_window.h"
#include "resources/fonts/fonts.h"
#include "resources/styles/styles.h"

namespace hebo {
namespace {

constexpr const char* kI18Template = ":/i18n/hebo-%1.qm";

}  // namespace

MainController::MainController(QObject* parent)
    : QObject(parent),
      connections_model_(new ConnectionsModel(this)),
      settings_manager_(new SettingsManager(this)),
      update_manager_(new UpdateManager()),
      update_thread_(new QThread()) {
  this->update_manager_->moveToThread(this->update_thread_);
  loadExternalFonts();
  this->installTranslators();
  this->initSignals();
  this->update_thread_->start();

  const bool night_mode = this->settings_manager_->isNightMode();
  QApplication::setStyle(new rusty::AdwaitaStyle(night_mode));
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

  window->setConnectionsModel(this->connections_model_);

  auto* settings_window = window->settingsWindow();
  Q_ASSERT(settings_window != nullptr);
  connect(this->settings_manager_, &SettingsManager::localeChanged,
          settings_window, &SettingsWindow::setLocale);
  connect(this->settings_manager_, &SettingsManager::retryConnectionsChanged,
          settings_window, &SettingsWindow::setRetryConnection);
  connect(this->settings_manager_, &SettingsManager::autoUpdateChanged,
          settings_window, &SettingsWindow::setAutoUpdate);
  connect(this->settings_manager_, &SettingsManager::nightModeChanged,
          settings_window, &SettingsWindow::setNightMode);

  connect(settings_window, &SettingsWindow::localeChanged,
          this->settings_manager_, &SettingsManager::setLocale);
  connect(settings_window, &SettingsWindow::retryConnectionChanged,
          this->settings_manager_, &SettingsManager::setRetryConnections);
  connect(settings_window, &SettingsWindow::autoUpdateChanged,
          this->settings_manager_, &SettingsManager::setAutoUpdate);
  connect(settings_window, &SettingsWindow::nightModeChanged,
          this->settings_manager_, &SettingsManager::setNightMode);

  settings_window->setLocale(this->settings_manager_->locale());
  settings_window->setRetryConnection(this->settings_manager_->retryConnections());
  settings_window->setAutoUpdate(this->settings_manager_->autoUpdate());
  settings_window->setNightMode(this->settings_manager_->isNightMode());
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

  connect(this->settings_manager_, &SettingsManager::nightModeChanged,
          this, &MainController::onNightModeChanged);
}

void loadExternalFonts() {
  for (const char* font : kExternalFonts) {
    QFontDatabase::addApplicationFont(font);
  }
}

void MainController::onNightModeChanged(bool night_mode) {
  QApplication::setStyle(new rusty::AdwaitaStyle(night_mode));
}

}  // namespace hebo
