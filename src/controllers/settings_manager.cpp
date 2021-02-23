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
  theme_names_ << tr("Light") << tr("Dark") << tr("Night");
  themes_ << "light" << "dark" << "night";
}

bool SettingsManager::sync() {
  this->settings_->sync();
  return this->settings_->status() == QSettings::NoError;
}

bool SettingsManager::autoUpdate() {
  return this->settings_->value(kAutoUpdate).toBool();
}

void SettingsManager::setAutoUpdate(bool enable) {
  qDebug() << __func__ << enable;
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

int SettingsManager::retryConnections() {
  return this->settings_->value(kMaxRetry, kDefaultRetries).toInt();
}

void SettingsManager::setRetryConnections(int retries) {
  qDebug() << __func__ << retries;
  this->settings_->setValue(kMaxRetry, retries);
  emit this->retryConnectionsChanged(retries);
}

int SettingsManager::themeId() {
  const QString theme = this->settings_->value(kTheme, kDefaultTheme).toString();
  const int index = this->themes_.indexOf(theme);
  Q_ASSERT(index > -1);
  return index;
}

void SettingsManager::setThemeId(int index) {
  qDebug() << __func__ << index;
  Q_ASSERT(index > -1 && index < this->themes_.length());
  this->settings_->setValue(kTheme, this->themes_.at(index));
  emit this->themeIdChanged(index);
}

}  // namespace hebo