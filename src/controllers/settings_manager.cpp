// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/settings_manager.h"

namespace hebo {
namespace {

constexpr const char* kAutoUpdate = "autoUpdate";
constexpr const char* kLocale = "locale";
constexpr const char* kDefaultLocale = "en_US";

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

QString SettingsManager::locale() {
  return this->settings_->value(kLocale, kDefaultLocale).toString();
}

QStringList SettingsManager::availableLocales() const {
  return {
    "en_US",
    "zh_CN"
  };
}

void SettingsManager::setLocale(const QString& locale) {
  this->settings_->setValue(kLocale, locale);
  emit this->localeChanged(locale);
}

}  // namespace hebo