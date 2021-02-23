// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_LOG_MANAGER_H_
#define HEBOUI_SRC_CONTROLLERS_LOG_MANAGER_H_

#include <QObject>

namespace hebo {

// Make access to mqtt service log.
class LogManager : public QObject {
  Q_OBJECT
  Q_PROPERTY(int pages READ getPages NOTIFY pagesUpdated)

 public:
  explicit LogManager(QObject* parent = nullptr);

  [[nodiscard]] int getPages() const;

  Q_INVOKABLE QString getLog(int page);

 signals:
  void pagesUpdated(int pages);

 private:
  int total_pages_{};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_LOG_MANAGER_H_
