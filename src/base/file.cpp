// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "base/file.h"

#include <QDebug>
#include <QFile>

namespace hebo {

QByteArray readBinaryFile(const QString& path) {
  QFile file(path);
  if (file.exists()) {
    if (!file.open(QIODevice::ReadOnly)) {
      qWarning() << "failed to open file:" << path;
      return {};
    }
    QByteArray bytes = file.readAll();
    file.close();
    return bytes;
  }
  qWarning() << "file not found:" << path;
  return {};
}

QString readTextFile(const QString& path) {
  QFile file(path);
  if (file.exists()) {
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
      qWarning() << "failed to open file:" << path;
      return "";
    }
    QString content = file.readAll();
    file.close();
    return content;
  }
  qWarning() << "file not found:" << path;
  return "";
}

bool writeBinaryFile(const QString& path, const QByteArray& bytes) {
  QFile file(path);
  if (!file.open(QIODevice::WriteOnly)) {
    qWarning() << "failed to open file:" << path;
    return false;
  }
  file.write(bytes);
  file.close();
  return true;
}

}  // namespace hebo