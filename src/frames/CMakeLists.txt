
set(SOURCES
    main_window.cpp
    main_window.h
    )

add_library(frames-static STATIC
    ${SOURCES}
    )
target_link_libraries(frames-static
    Qt5::Widgets
    )