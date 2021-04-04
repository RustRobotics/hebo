// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_CONTROLLERS_LOG_MANAGER_H_
#define HEBO_SRC_CONTROLLERS_LOG_MANAGER_H_

#include <QObject>

namespace hebo {

// Make access to mqtt service log.
class LogManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(QString log READ getLog NOTIFY logUpdated)

 public:
  explicit LogManager(QObject* parent = nullptr);

  const QString& getLog();

 signals:
  void logUpdated(const QString& log);

 private:
  QString log_{"Hello, rust"};
};

}  // namespace hebo

#endif  // HEBO_SRC_CONTROLLERS_LOG_MANAGER_H_
