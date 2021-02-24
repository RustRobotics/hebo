// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_BASE_FILE_H_
#define HEBOUI_SRC_BASE_FILE_H_

#include <QString>

namespace hebo {

QByteArray readBinaryFile(const QString& path);

QString readTextFile(const QString& path);

bool writeBinaryFile(const QString& path, const QByteArray& bytes);

}  // namespace hebo

#endif  // HEBOUI_SRC_BASE_FILE_H_
