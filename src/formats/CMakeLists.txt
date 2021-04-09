
set(SOURCE_FILES
    color_palette.cpp
    color_palette.h
    connect_config.cpp
    connect_config.h
    license_parser.cpp
    license_parser.h
    )
set(TEST_FILES
    color_palette_test.cpp
    license_parser_test.cpp

    ../resources/misc/misc.qrc
    )

add_library(formats-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(formats-static
    Qt5::Core
    Qt5::Gui

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