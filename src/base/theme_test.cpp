// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <gtest/gtest.h>
#include <QDebug>

#include "base/theme.h"

namespace hebo {

TEST(ThemeTest, TestReadTheme) {
  const QString content = readThemeFile(":/tests/night-theme.css");
  ASSERT_FALSE(content.isEmpty());
  qDebug() << qPrintable(content);
}

}  // namespace hebo