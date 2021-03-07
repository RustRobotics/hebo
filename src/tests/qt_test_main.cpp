// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include <ConsoleAppender.h>
#include <Logger.h>
#include <QGuiApplication>
#include <gtest/gtest.h>

int main(int argc, char** argv) {
  ::testing::InitGoogleTest(&argc, argv);
  cuteLogger->registerAppender(new ConsoleAppender());
  QGuiApplication app(argc, argv);
  return RUN_ALL_TESTS();
}
