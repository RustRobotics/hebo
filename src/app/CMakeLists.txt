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

add_executable(mqtt-client-demo
    mqtt_client.cpp
    )
target_link_libraries(mqtt-client-demo
    Qt5::Core

    mqtt_cpp_iface
    )