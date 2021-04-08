// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FORMATS_COLOR_PALETTE_H_
#define HEBO_SRC_FORMATS_COLOR_PALETTE_H_

#include <QColor>
#include <QVector>

namespace hebo {

using ColorPalette = QVector<QColor>;

ColorPalette parseColorPalette(const QString& json_file, bool* ok);

}  // namespace hebo

#endif  // HEBO_SRC_FORMATS_COLOR_PALETTE_H_
