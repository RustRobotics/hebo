
set(SOURCE_FILES
    font_button.cpp
    font_button.h
    form_section.cpp
    form_section.h
    form_section_title.cpp
    form_section_title.h
    left_panel_button.cpp
    left_panel_button.h
    round_font_button.cpp
    round_font_button.h
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