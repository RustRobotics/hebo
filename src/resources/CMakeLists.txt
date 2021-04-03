
set(SOURCE_FILES
    fonts/fonts.h
    images/images.h
    )

add_library(resources-static STATIC
    ${SOURCE_FILES}
    )