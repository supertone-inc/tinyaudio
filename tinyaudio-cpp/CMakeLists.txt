cmake_minimum_required(VERSION 3.20)

file(STRINGS ${CMAKE_CURRENT_SOURCE_DIR}/../VERSION VERSION)

project(
    tinyaudio
    VERSION ${VERSION}
    DESCRIPTION "A tiny library for audio I/O."
    HOMEPAGE_URL "https://github.com/supertone-inc/tinyaudio/tree/main/tinyaudio-cpp"
    LANGUAGES C CXX
)

add_library(tinyaudio src/tinyaudio.cpp)
set_target_properties(tinyaudio PROPERTIES
    VERSION ${PROJECT_VERSION}
    CXX_STANDARD 17
)
target_include_directories(tinyaudio PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

include(cmake/miniaudio.cmake)
target_link_libraries(tinyaudio miniaudio)

if(TINYAUDIO_BUILD_TESTS)
    include(cmake/doctest.cmake)
    add_executable(tinyaudio-test src/test-main.cpp $<TARGET_OBJECTS:tinyaudio>)
    set_target_properties(tinyaudio-test PROPERTIES CXX_STANDARD 17)
    target_link_libraries(tinyaudio-test doctest)

    if(UNIX AND NOT APPLE)
        target_link_libraries(tinyaudio-test dl pthread m)
    endif()

    target_compile_definitions(tinyaudio PRIVATE TINYAUDIO_BUILD_TESTS)
    target_link_libraries(tinyaudio doctest)

    install(TARGETS tinyaudio-test)
endif()