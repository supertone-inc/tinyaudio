set(MINIAUDIO_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../miniaudio)

execute_process(
    COMMAND git submodule update --init --recursive ${MINIAUDIO_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_library(miniaudio INTERFACE)
target_include_directories(miniaudio INTERFACE ${MINIAUDIO_DIR}/extras/miniaudio_split)
target_sources(miniaudio INTERFACE ${MINIAUDIO_DIR}/extras/miniaudio_split/miniaudio.c)

if(UNIX)
    target_compile_options(miniaudio INTERFACE -Wno-deprecated-declarations)
endif()
