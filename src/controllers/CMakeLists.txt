set(SOURCES
    main_controller.cpp
    main_controller.h
    shell.cpp
    shell.h

    )

add_library(controllers-static STATIC
    ${SOURCES}
    )
target_link_libraries(controllers-static
    Qt5::Gui
    Qt5::Qml

    CuteLogger

    ui-static
    )