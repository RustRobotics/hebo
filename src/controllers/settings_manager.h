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

 public:
  explicit SettingsManager(QObject* parent = nullptr);

  bool sync();

  bool autoUpdate();

  QString locale();

  [[nodiscard]] QStringList availableLocales() const;

  int retryConnections();

 public slots:
  void setAutoUpdate(bool enable);

  void setLocale(const QString& locale);

  void setRetryConnections(int retries);

 signals:
  void autoUpdateChanged(bool enable);

  void localeChanged(const QString& locale);

  void retryConnectionsChanged(int retries);

 private:
  QSettings* settings_;
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
