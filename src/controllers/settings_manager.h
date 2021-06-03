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
  Q_PROPERTY(bool autoUpdate READ autoUpdate WRITE setAutoUpdate NOTIFY autoUpdateChanged);
  Q_PROPERTY(int retryConnections READ retryConnections WRITE setRetryConnections
             NOTIFY retryConnectionsChanged);
  Q_PROPERTY(QString locale READ locale WRITE setLocale NOTIFY localeChanged);
  Q_PROPERTY(bool nightMode READ isNightMode WRITE setNightMode NOTIFY nightModeChanged);
 public:
  explicit SettingsManager(QObject* parent = nullptr);

  bool sync();

  bool autoUpdate();

  int retryConnections();

  QString locale();

  bool isNightMode();

 public slots:
  void setAutoUpdate(bool enable);

  void setRetryConnections(int retries);

  void setLocale(const QString& locale);

  void setNightMode(bool night_mode);

 signals:
  void autoUpdateChanged(bool enable);

  void retryConnectionsChanged(int retries);

  void localeChanged(const QString& locale);

  void nightModeChanged(bool night_mode);

 private:
  QSettings* settings_;
};

}  // namespace hebo

#endif  // HEBO_SRC_CONTROLLERS_SETTINGS_MANAGER_H_
