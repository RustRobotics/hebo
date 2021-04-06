
set(SOURCE_FILES
    connect_config.cpp
    connect_config.h
    )

add_library(formats-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(formats-static Qt5::Core)