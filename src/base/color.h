// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_BASE_COLOR_H_
#define HEBO_SRC_BASE_COLOR_H_

#include <QColor>
#include <QString>

namespace hebo {

QColor parseColor(QString val);

QColor randomColor();

}  // namespace hebo

#endif  // HEBO_SRC_BASE_COLOR_H_
