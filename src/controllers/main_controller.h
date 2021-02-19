// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
#define HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_

#include <QObject>

namespace hebo {

class MainController : public QObject {
  Q_OBJECT
 public:
  explicit MainController(QObject* parent = nullptr);

  void showMainWindow();
};

}  // namespace hebo

#endif  // HEBOUI_SRC_CONTROLLERS_MAIN_CONTROLLER_H_
