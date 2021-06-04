
set(SOURCE
    color.cpp
    color.h
    file.cpp
    file.h
    random.cpp
    random.h
    )
set(TEST_FILES
    color_test.cpp
    random_test.cpp

    tests/tests.qrc
    )

add_library(base-static STATIC
    ${SOURCE}
    )
target_link_libraries(base-static
    Qt5::Core
    Qt5::Gui
    )

if (BUILD_TESTING)
  add_executable(base-tests
      ${TEST_FILES}
      )
  target_link_libraries(base-tests
      Qt5::Core

      base-static
      tests-static
      )
  add_test(base-tests base-tests)
endif ()