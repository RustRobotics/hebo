// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/log_manager.h"

namespace hebo {

LogManager::LogManager(QObject* parent) : QObject(parent) {

}

int LogManager::getPages() const {
  return this->total_pages_;
}

QString LogManager::getLog(int page) {
  Q_ASSERT(page < this->total_pages_);
  return QString();
}

}  // namespace hebo