// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_SETTINGS_MANAGER_H_

#include <QObject>
#include <QSettings>

namespace hebo {

class SettingsManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(bool autoUpdate READ autoUpdate WRITE setAutoUpdate NOTIFY autoUpdateChanged)
  Q_PROPERTY(QString locale READ locale WRITE setLocale NOTIFY localeChanged)
  Q_PROPERTY(QStringList availableLocales READ availableLocales);
  Q_PROPERTY(int retryConnections READ retryConnections WRITE setRetryConnections
             NOTIFY retryConnectionsChanged)
  Q_PROPERTY(QStringList themeNames READ themeNames NOTIFY themeNamesChanged)
  Q_PROPERTY(int themeId READ themeId WRITE setThemeId NOTIFY themeIdChanged)

 public:
  explicit SettingsManager(QObject* parent = nullptr);

  bool sync();

  bool autoUpdate();

  QString locale();

  int retryConnections();

  [[nodiscard]] QStringList availableLocales() const;

  [[nodiscard]] const QStringList& themeNames() const {
    return this->theme_names_;
  }

  int themeId();

 public slots:
  void setAutoUpdate(bool enable);

  void setLocale(const QString& locale);

  void setRetryConnections(int retries);

  void setThemeId(int index);

 signals:
  void autoUpdateChanged(bool enable);

  void localeChanged(const QString& locale);

  void retryConnectionsChanged(int retries);

  void themeNamesChanged(const QStringList& list);

  void themeIdChanged(int themeId);

 private:
  QSettings* settings_;
  QStringList theme_names_{};
  QStringList themes_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
