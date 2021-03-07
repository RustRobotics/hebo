find_package(Qt5 5.12 COMPONENTS Core Gui Network Svg Qml Quick REQUIRED)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

# Set current directory as root of header files.
include_directories(${CMAKE_CURRENT_SOURCE_DIR})

add_subdirectory(third_party/CuteLogger)
add_subdirectory(third_party/mqtt_cpp)

add_subdirectory(app)
add_subdirectory(base)
add_subdirectory(controllers)
add_subdirectory(mqtt)
add_subdirectory(tests)
add_subdirectory(ui)
