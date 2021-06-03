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
constexpr const char* kNightModeName = "nightMode";
constexpr bool kDefaultNightMode = true;

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

bool SettingsManager::isNightMode() {
  return this->settings_->value(kNightModeName, kDefaultNightMode).toBool();
}

void SettingsManager::setNightMode(bool night_mode) {
  this->settings_->setValue(kNightModeName, night_mode);
  emit this->nightModeChanged(night_mode);
}

}  // namespace hebo
