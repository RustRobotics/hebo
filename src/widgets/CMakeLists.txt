
set(SOURCE_FILES
    round_font_button.cpp
    round_font_button.h
    )

add_library(widgets-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(widgets-static
    Qt5::Widgets
    )