
set(SOURCE_FILES
    font_icon_button.cpp
    font_icon_button.h
    form_section.cpp
    form_section.h
    form_section_title.cpp
    form_section_title.h
    integer_line_edit.cpp
    integer_line_edit.h
    left_panel_button.cpp
    left_panel_button.h
    round_font_button.cpp
    round_font_button.h
    spin_box.cpp
    spin_box.h
    switch_button.cpp
    switch_button.h
    text_button.cpp
    text_button.h
    )

add_library(widgets-static STATIC
    ${SOURCE_FILES}
    )
target_link_libraries(widgets-static
    Qt5::Widgets
    )