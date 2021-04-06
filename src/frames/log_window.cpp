// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/log_window.h"

namespace hebo {

LogWindow::LogWindow(QWidget* parent) : QPlainTextEdit(parent) {
  this->initUi();
}

void LogWindow::initUi() {
  this->setWindowTitle(tr("Logs"));
  this->setReadOnly(true);
}

}  // namespace hebo