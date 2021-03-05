
set(SOURCE
    color.cpp
    color.h
    file.cpp
    file.h
    random.cpp
    random.h
    )

add_library(base-static STATIC
    ${SOURCE}
    )
target_link_libraries(base-static
    Qt5::Core
    Qt5::Gui
    )