
set(SOURCE
    connect_config.cpp
    connect_config.h
    mqtt_client.cpp
    mqtt_client.h
    )

add_library(mqtt-static STATIC
    ${SOURCE}
    )
target_link_libraries(mqtt-static
    Qt5::Core
    Qt5::Gui

    mqtt_cpp_iface

    base-static
    )
