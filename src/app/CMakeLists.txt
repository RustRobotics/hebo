
add_executable(hebo
    hebo_main.cpp
    )
target_link_libraries(hebo
    Qt5::Core
    controllers-static
    )