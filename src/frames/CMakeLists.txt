
set(SOURCES
    delegates/connections_delegate.cpp
    delegates/connections_delegate.h
    delegates/messages_delegate.cpp
    delegates/messages_delegate.h
    delegates/software_license_delegate.cpp
    delegates/software_license_delegate.h

    internal/color_chooser_window.cpp
    internal/color_chooser_window.h
    internal/connection_form.cpp
    internal/connection_form.h

    models/protocol_model.cpp
    models/protocol_model.h
    models/qos_model.cpp
    models/qos_model.h
    models/software_license_model.cpp
    models/software_license_model.h
    models/version_model.cpp
    models/version_model.h

    about_window.cpp
    about_window.h
    bag_window.cpp
    bag_window.h
    benchmark_window.cpp
    benchmark_window.h
    client_frame.cpp
    client_frame.h
    left_panel.cpp
    left_panel.h
    log_window.cpp
    log_window.h
    main_window.cpp
    main_window.h
    connections_window.cpp
    connections_window.h
    new_connection_window.cpp
    new_connection_window.h
    settings_window.cpp
    settings_window.h
    software_license_window.cpp
    software_license_window.h
    )

add_library(frames-static STATIC
    ${SOURCES}
    )
target_link_libraries(frames-static
    Qt5::Widgets

    config-static
    widgets-static
    )