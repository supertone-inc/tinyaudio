set(MINIAUDIO_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../miniaudio)

execute_process(
    COMMAND git submodule update --init --recursive ${MINIAUDIO_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_library(miniaudio ${MINIAUDIO_DIR}/extras/miniaudio_split/miniaudio.c)
target_include_directories(miniaudio PUBLIC ${MINIAUDIO_DIR}/extras/miniaudio_split)
target_compile_options(miniaudio PRIVATE -Wno-deprecated-declarations)
