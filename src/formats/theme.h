// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FORMATS_THEME_H_
#define HEBO_SRC_FORMATS_THEME_H_

#include <cstdint>
#include <QString>
#include <QMetaType>

namespace hebo {

enum class ThemeType : uint8_t {
  kDay = 0,
  kNight,
};

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::ThemeType);

#endif  // HEBO_SRC_FORMATS_THEME_H_
