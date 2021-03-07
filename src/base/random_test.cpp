// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <gtest/gtest.h>
#include <QDebug>
#include <QSet>

#include "base/random.h"

namespace hebo {

TEST(RandomTest, TestGenerateConfigId) {
  const QString config_id = generateConfigId();
  ASSERT_EQ(config_id.length(), 36);

  QSet<QString> set{};
  for (int i = 0; i < 100; ++i) {
    const QString id = generateConfigId();
    ASSERT_FALSE(set.contains(id));
    set.insert(id);
  }
}

}  // namespace hebo