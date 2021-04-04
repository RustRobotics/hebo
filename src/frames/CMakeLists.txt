
set(SOURCES
    about_windows.cpp
    about_windows.h
    bag_window.cpp
    bag_window.h
    benchmark_window.cpp
    benchmark_window.h
    left_panel.cpp
    left_panel.h
    log_window.cpp
    log_window.h
    main_window.cpp
    main_window.h
    new_connection_window.cpp
    new_connection_window.h
    settings_window.cpp
    settings_window.h
    )

add_library(frames-static STATIC
    ${SOURCES}
    )
target_link_libraries(frames-static
    Qt5::Widgets
    )