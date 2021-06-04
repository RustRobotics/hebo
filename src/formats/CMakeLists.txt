
set(SOURCE_FILES
    color_palette.cpp
    color_palette.h
    connect_config.cpp
    connect_config.h
    payload_type.cpp
    payload_type.h
    )
set(TEST_FILES
    color_palette_test.cpp

    ../resources/misc/misc.qrc
    )

add_library(formats-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(formats-static
    Qt5::Core
    Qt5::Gui
    rusty-static

    base-static
    )

if (BUILD_TESTING)
  add_executable(formats-tests
      ${TEST_FILES}
      )
  target_link_libraries(formats-tests
      Qt5::Core

      formats-static
      tests-static
      )
  add_test(formats-tests formats-tests)
endif ()