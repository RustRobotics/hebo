
function(generate_config_file)
  if (CMAKE_BUILD_TYPE MATCHES Debug)
    set(HEBO_RESOURCES_DIR ${CMAKE_SOURCE_DIR}/resources)
  else ()
    set(HEBO_RESOURCES_DIR ${CMAKE_INSTALL_PREFIX}/resources)
  endif ()
  set(HEBO_VERSION ${PROJECT_VERSION})

  execute_process(
      COMMAND git rev-parse --short HEAD
      OUTPUT_VARIABLE HEBO_BUILD_ID
      OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  message("HEBO_BUILD_ID: ${HEBO_BUILD_ID}")
  string(TIMESTAMP HEBO_BUILD_DATE "%Y%m%d")
  configure_file(config.h.in ${CMAKE_CURRENT_BINARY_DIR}/config.h
      @ONLY
      )
endfunction()

generate_config_file()

## Custom library
set(GENERATED_CONFIG_FILES
    ${CMAKE_CURRENT_BINARY_DIR}/config.h
    )

# TODO(Shaohua): Regenerate at runtime
add_custom_command(OUTPUT ${GENERATED_CONFIG_FILES}
    DEPENDS config.h.in
    )

add_custom_target(config-generate-files
    DEPENDS ${GENERATED_CONFIG_FILES}
    )

add_library(config-static STATIC
    ${GENERATED_CONFIG_FILES}
    )
add_dependencies(config-static
    config-generate-files
    )
target_include_directories(config-static PUBLIC
    ${CMAKE_CURRENT_BINARY_DIR}/..
    )