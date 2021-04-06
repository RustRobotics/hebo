
set(SOURCE_FILES
    fonts/fonts.h
    images/images.h
    misc/misc.h
    styles/styles.h
    )

add_library(resources-static STATIC
    ${SOURCE_FILES}
    )