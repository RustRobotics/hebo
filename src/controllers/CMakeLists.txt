set(SOURCES
    main_controller.cpp
    main_controller.h
    settings_manager.cpp
    settings_manager.h
    shell.cpp
    shell.h
    update_manager.cpp
    update_manager.h
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