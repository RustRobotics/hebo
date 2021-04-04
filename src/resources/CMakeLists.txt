
set(SOURCE_FILES
    fonts/fonts.h
    images/images.h
    styles/styles.h
    )

add_library(resources-static STATIC
    ${SOURCE_FILES}
    )