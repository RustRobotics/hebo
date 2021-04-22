// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_BASE_THEME_H_
#define HEBO_SRC_BASE_THEME_H_

#include <QString>

namespace hebo {

// Read theme file and parse CSS_IMPORT macro and import dependents.
QString readThemeFile(const QString& file);

}  // namespace hebo

#endif  // HEBO_SRC_BASE_THEME_H_
