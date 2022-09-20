cmake_minimum_required(VERSION 3.20)

project(tinyaudio)

include(miniaudio.cmake)
include(doctest.cmake)

if(TINYAUDIO_BUILD_TESTS)
    add_library(tinyaudio OBJECT src/tinyaudio.cpp)
    add_executable(tinyaudio-test src/test-main.cpp $<TARGET_OBJECTS:tinyaudio>)
    set_target_properties(tinyaudio-test PROPERTIES CXX_STANDARD 11)
    target_link_libraries(tinyaudio-test miniaudio)
    target_link_libraries(tinyaudio-test doctest)
    install(TARGETS tinyaudio-test)
else()
    add_library(tinyaudio src/tinyaudio.cpp)
    target_compile_definitions(tinyaudio PRIVATE DOCTEST_CONFIG_DISABLE)
endif()

set_target_properties(tinyaudio PROPERTIES CXX_STANDARD 11)
target_include_directories(tinyaudio PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(tinyaudio miniaudio)
target_link_libraries(tinyaudio doctest)
