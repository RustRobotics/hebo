set(RESOURCES_FILES
    ../resources/images/images.qrc
    ${CMAKE_BINARY_DIR}/i18n/hebo.qrc
    )

add_executable(hebo
    hebo_main.cpp
    ${RESOURCES_FILES}
    )
target_link_libraries(hebo
    Qt5::Core
    controllers-static
    )
add_dependencies(hebo generate-i18n-files)

add_executable(mqtt-client-demo
    mqtt_client_demo.cpp
    )
target_link_libraries(mqtt-client-demo
    Qt5::Core

    mqtt_cpp_iface
    )
