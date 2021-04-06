// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <gtest/gtest.h>

#include "formats/license_parser.h"
#include "resources/misc/misc.h"

namespace publisher {

TEST(LicenseParserTest, TestParseLicenseFile) {
  const auto list = parseAppLicense(kMiscSoftwareLicense);
  ASSERT_FALSE(list.isEmpty());
}

}  // namespace publisher