set(UI_FILES
    ../ui/ui.qrc
    )

add_executable(hebo
    hebo_main.cpp
    ${UI_FILES}
    )
target_link_libraries(hebo
    Qt5::Core
    controllers-static
    )