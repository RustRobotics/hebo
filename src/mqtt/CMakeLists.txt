
set(SOURCE
    internal_client.cpp
    internal_client.h
    message_stream_model.cpp
    message_stream_model.h
    mqtt_client.cpp
    mqtt_client.h
    subscription_model.cpp
    subscription_model.h
    )

add_library(mqtt-static STATIC
    ${SOURCE}
    )
target_link_libraries(mqtt-static
    Qt5::Core
    Qt5::Gui

    mqtt_cpp_iface

    base-static
    formats-static
    )
