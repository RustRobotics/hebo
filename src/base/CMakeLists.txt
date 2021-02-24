
set(SOURCE
    file.cpp
    file.h
    )

add_library(base-static STATIC
    ${SOURCE}
    )
target_link_libraries(base-static
    Qt5::Core
    )