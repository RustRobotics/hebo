// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/settings_manager.h"

#include <QDebug>

namespace hebo {
namespace {

constexpr const char* kAutoUpdate = "autoUpdate";
constexpr const char* kLocale = "locale";
constexpr const char* kDefaultLocale = "en_US";
constexpr const char* kMaxRetry = "retryConnections";
constexpr int kDefaultRetries = 3;

constexpr const char* kTheme = "theme";
constexpr const char* kDefaultTheme = "light";

}  // namespace

SettingsManager::SettingsManager(QObject* parent)
    : QObject(parent),
      settings_(new QSettings(this)) {
}

bool SettingsManager::sync() {
  this->settings_->sync();
  return this->settings_->status() == QSettings::NoError;
}

bool SettingsManager::autoUpdate() {
  return this->settings_->value(kAutoUpdate).toBool();
}

void SettingsManager::setAutoUpdate(bool enable) {
  this->settings_->setValue(kAutoUpdate, enable);
  emit this->autoUpdateChanged(enable);
}

int SettingsManager::retryConnections() {
  return this->settings_->value(kMaxRetry, kDefaultRetries).toInt();
}

void SettingsManager::setRetryConnections(int retries) {
  this->settings_->setValue(kMaxRetry, retries);
  emit this->retryConnectionsChanged(retries);
}

QString SettingsManager::locale() {
  return this->settings_->value(kLocale, kDefaultLocale).toString();
}

void SettingsManager::setLocale(const QString& locale) {
  this->settings_->setValue(kLocale, locale);
  emit this->localeChanged(locale);
}

QString SettingsManager::theme() {
  return this->settings_->value(kTheme, kDefaultTheme).toString();
}

void SettingsManager::setTheme(const QString& theme) {
  this->settings_->setValue(kTheme, theme);
  emit this->themeChanged(theme);
}

}  // namespace hebo
