// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <gtest/gtest.h>

#include "base/color.h"

namespace hebo {

TEST(ColorTest, TestRandomColor) {
  ASSERT_NE(randomColor(), randomColor());
}

}  // namespace hebo