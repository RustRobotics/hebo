// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/log_manager.h"

namespace hebo {

LogManager::LogManager(QObject* parent) : QObject(parent) {
}

QString LogManager::getLogFile(const QString& conn_id) {
  Q_UNUSED(conn_id);
  return QString();
}

}  // namespace hebo