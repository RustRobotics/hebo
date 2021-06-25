
set(SOURCE_FILES
    font_icon_button.cpp
    font_icon_button.h
    hsv_color_picker.cpp
    hsv_color_picker.h
    left_panel_button.cpp
    left_panel_button.h
    round_font_button.cpp
    round_font_button.h
    text_button.cpp
    text_button.h
    )

add_library(widgets-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(widgets-static
    Qt5::Widgets
    )