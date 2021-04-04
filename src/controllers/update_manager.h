// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_CONTROLLERS_UPDATE_MANAGER_H_
#define HEBO_SRC_CONTROLLERS_UPDATE_MANAGER_H_

#include <QObject>

namespace hebo {

class UpdateManager : public QObject {
  Q_OBJECT

 public:
  explicit UpdateManager(QObject* parent = nullptr);

 signals:
  void checkUpdate();
  void checkUpdateResult(bool has_new_version, const QString& version);

 private slots:
  void doCheckUpdate();
};

}  // namespace hebo

#endif  // HEBO_SRC_CONTROLLERS_UPDATE_MANAGER_H_
