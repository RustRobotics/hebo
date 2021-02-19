set(SOURCES
    shell.cpp
    shell.h
    )

add_library(controllers-static STATIC
    ${SOURCES}
    )
target_link_libraries(controllers-static
    Qt5::Core
    Qt5::Gui

    CuteLogger
    )