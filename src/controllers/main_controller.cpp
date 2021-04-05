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
#include "resources/fonts/fonts.h"

namespace hebo {
namespace {

constexpr const char* kI18Template = ":/i18n/hebo-%1.qm";

}  // namespace

MainController::MainController(QObject* parent)
    : QObject(parent),
      updater_thread_(new QThread()) {
  this->installTranslators();
  loadExternalFonts();
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
  main_window->resize(1020, 720);
  main_window->show();
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

void loadExternalFonts() {
  for (const char* font : kExternalFonts) {
    QFontDatabase::addApplicationFont(font);
  }
}

}  // namespace hebo
