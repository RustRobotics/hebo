// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <gtest/gtest.h>

#include "formats/color_palette.h"

namespace hebo {

TEST(ColorPaletteTest, TestParsePalette) {
  const QString json_file{":/tests/color-palette.json"};
  bool ok;
  ColorPalette palette = parseColorPalette(json_file, &ok);
  ASSERT_TRUE(ok);
  ASSERT_EQ(palette.length(), 5);
}

}  // namespace hebo