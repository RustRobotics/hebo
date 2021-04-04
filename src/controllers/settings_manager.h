// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
#define HEBO_SRC_CONTROLLERS_SETTINGS_MANAGER_H_

#include <QObject>
#include <QSettings>

namespace hebo {

class SettingsManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(bool autoUpdate READ autoUpdate WRITE setAutoUpdate NOTIFY autoUpdateChanged)

  Q_PROPERTY(int retryConnections READ retryConnections WRITE setRetryConnections
             NOTIFY retryConnectionsChanged)

  Q_PROPERTY(int localeIndex READ localeIndex WRITE setLocaleIndex NOTIFY localeIndexChanged)
  Q_PROPERTY(QStringList localeNames READ localeNames NOTIFY localeNamesChanged);

  Q_PROPERTY(QStringList themeNames READ themeNames NOTIFY themeNamesChanged)
  Q_PROPERTY(int themeIndex READ themeIndex WRITE setThemeIndex NOTIFY themeIndexChanged)

 public:
  explicit SettingsManager(QObject* parent = nullptr);

  bool sync();

  bool autoUpdate();

  int retryConnections();

  [[nodiscard]] const QStringList& localeNames() const {
    return this->locale_names_;
  }

  int localeIndex();

  [[nodiscard]] const QStringList& themeNames() const {
    return this->theme_names_;
  }

  int themeIndex();

 public slots:
  void setAutoUpdate(bool enable);

  void setRetryConnections(int retries);

  void setLocaleIndex(int index);

  void setThemeIndex(int index);

 signals:
  void autoUpdateChanged(bool enable);

  void retryConnectionsChanged(int retries);

  void localeNamesChanged(const QStringList& list);

  void localeIndexChanged(int index);

  void themeNamesChanged(const QStringList& list);

  void themeIndexChanged(int index);

 private:
  QSettings* settings_;
  QStringList locale_names_{};
  QStringList locales_{};

  QStringList theme_names_{};
  QStringList themes_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
