cmake_minimum_required(VERSION 3.14)
project(HeboUi VERSION 0.1.1)

set(CMAKE_CONFIGURATION_TYPES Debug Release)
if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE "Release" CACHE STRING "Default build type: Release" FORCE)
endif ()
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Generate compile_commands.json project file.
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

option(ENABLE_SANITIZER "Enable sanitizer in debug mode" OFF)

if (CMAKE_BUILD_TYPE MATCHES Debug)
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Wall -Werror -Wextra")

  # See https://fuzzing-project.org/tutorial-cflags.html.
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Werror -Wextra \
  -fPIC -fno-omit-frame-pointer")
  # -fno-rtti, Disable real-time type information
  # -fno-exceptions Disable exceptions

  if (ENABLE_SANITIZER)
    message("Enable memory sanitizer")

    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address,leak,undefined \
  -fsanitize-address-use-after-scope")
    set(CMAKE_CXX_LINK_LIBRARY_FLAG "${CMAKE_CXX_LINK_LIBRARY_FLAG} -fsanitize=address,leak,undefined")
    # -fsanitize=address, Enable memory address sanitizer.
    # -fsanitize=leak, Enable memory leak sanitizer.
    # -fsanitize=memory, Enable detecting uninitialized memory sanitizer.
    # -fsanitize=undefined, Check undefined behaviors.
    #  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=memory")
    #  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wimplicit-fallthrough=1")
  endif ()

  # Enable Qt builtin debug mode
  add_definitions("-DQT_MESSAGELOGCONTEXT")
else ()
  # -Wl, -O2 Enable linker optimizations
  #-Wl, --gc-sections Remove unused code resulting from -fdsta-sections and -ffunction-sections
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -O2 -Wl,-O1 -Wl,--gc-sections")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -O2 -Wl,-O1 -Wl,--gc-sections -Wno-unused-result")
endif ()

if (BUILD_TESTING)
  # Enable CTest
  enable_testing()
endif ()

add_subdirectory(src)

add_custom_target(generate-deps
    COMMAND mkdir -pv ${CMAKE_BINARY_DIR}/deps
    COMMAND cmake --graphviz=deps/deps.dot ..
    COMMAND dot -Tsvg -o deps/deps.svg deps/deps.dot
    COMMAND setsid xdg-open deps/deps.svg
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
