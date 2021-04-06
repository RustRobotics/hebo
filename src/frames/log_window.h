// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_LOG_WINDOW_H_
#define HEBO_SRC_FRAMES_LOG_WINDOW_H_

#include <QPlainTextEdit>

namespace hebo {

class LogWindow : public QPlainTextEdit {
  Q_OBJECT
 public:
  explicit LogWindow(QWidget* parent = nullptr);

 private:
  void initUi();
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_LOG_WINDOW_H_
