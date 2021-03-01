
find_package(Qt5LinguistTools REQUIRED)

# Generate .qm files from .ts files.
file(GLOB TS_FILES ${CMAKE_CURRENT_SOURCE_DIR}/hebo-*.ts)
qt5_add_translation(QM_FILES ${TS_FILES})
add_custom_target(generate-i18n-files DEPENDS ${QM_FILES})

set(I18N_RESOURCES "")
foreach(qm_file ${QM_FILES})
  get_filename_component(qm_file_alias ${qm_file} NAME)
  set(I18N_RESOURCES "${I18N_RESOURCES}\n    <file alias=\"${qm_file_alias}\">${qm_file}</file>")
endforeach()

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/hebo.qrc.in
    ${CMAKE_CURRENT_BINARY_DIR}/hebo.qrc
    @ONLY)
