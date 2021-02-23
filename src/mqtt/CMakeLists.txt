
set(SOURCE
    conn_info.cpp
    conn_info.h
    mqtt_client.cpp
    mqtt_client.h
    )

add_library(mqtt-static STATIC
    ${SOURCE}
    )
target_link_libraries(mqtt-static
    Qt5::Core

    mqtt_cpp_iface
    )
