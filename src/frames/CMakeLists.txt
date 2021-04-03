
set(SOURCES
    about_windows.cpp
    about_windows.h
    main_window.cpp
    main_window.h
    settings_window.cpp
    settings_window.h
    )

add_library(frames-static STATIC
    ${SOURCES}
    )
target_link_libraries(frames-static
    Qt5::Widgets
    )