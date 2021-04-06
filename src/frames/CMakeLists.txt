
set(SOURCES
    delegates/connections_delegate.cpp
    delegates/connections_delegate.h

    internal/connection_form.cpp
    internal/connection_form.h
    internal/connections_list_view.cpp
    internal/connections_list_view.h

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
    )

add_library(frames-static STATIC
    ${SOURCES}
    )
target_link_libraries(frames-static
    Qt5::Widgets

    widgets-static
    )