
set(SOURCES
    qt_test_main.cpp
    )

if (BUILD_TESTING)
  find_package(GTest REQUIRED)
  add_library(tests-static
    ${SOURCES}
    )
  target_link_libraries(tests-static
    GTest::gtest
    Qt5::Core
    Qt5::Gui
    Qt5::Widgets

    CuteLogger
    )
endif()
