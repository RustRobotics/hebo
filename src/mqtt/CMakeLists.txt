
set(SOURCE
    connection_model.cpp
    connection_model.h
    connection_info.cpp
    connection_info.h
    connection_state.cpp
    connection_state.h
    mqtt_client.cpp
    mqtt_client.h

    contact_model.cpp
    contact_model.h
    )

add_library(mqtt-static STATIC
    ${SOURCE}
    )
target_link_libraries(mqtt-static
    Qt5::Core

    mqtt_cpp_iface

    base-static
    )
