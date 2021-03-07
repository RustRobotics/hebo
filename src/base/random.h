// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_BASE_RANDOM_H_
#define HEBOUI_SRC_BASE_RANDOM_H_

#include <QString>

namespace hebo {

QString randomClientId();

QString generateConfigId();

}  // namespace hebo

#endif  // HEBOUI_SRC_BASE_RANDOM_H_
