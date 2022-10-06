cmake_minimum_required(VERSION 3.20)

file(STRINGS ${CMAKE_CURRENT_SOURCE_DIR}/../VERSION VERSION)

project(
    tinyaudio
    VERSION ${VERSION}
    DESCRIPTION "A tiny library for audio I/O."
    HOMEPAGE_URL "https://github.com/supertone-inc/tinyaudio/tree/main/tinyaudio-cpp"
    LANGUAGES C CXX
)

include(cmake/miniaudio.cmake)

if(TINYAUDIO_BUILD_TESTS)
    include(cmake/doctest.cmake)

    add_library(tinyaudio OBJECT src/tinyaudio.cpp)
    target_compile_definitions(tinyaudio PRIVATE TINYAUDIO_BUILD_TESTS)
    target_link_libraries(tinyaudio doctest)

    add_executable(tinyaudio-test src/test-main.cpp $<TARGET_OBJECTS:tinyaudio>)
    set_target_properties(tinyaudio-test PROPERTIES CXX_STANDARD 17)
    target_link_libraries(tinyaudio-test miniaudio)
    target_link_libraries(tinyaudio-test doctest)

    if(UNIX AND NOT APPLE)
        target_link_libraries(tinyaudio-test dl pthread m)
    endif()

    install(TARGETS tinyaudio-test)
else()
    add_library(tinyaudio src/tinyaudio.cpp)
endif()

set_target_properties(tinyaudio PROPERTIES
    VERSION ${PROJECT_VERSION}
    CXX_STANDARD 17
)
target_include_directories(tinyaudio PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(tinyaudio miniaudio)
