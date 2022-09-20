cmake_minimum_required(VERSION 3.20)

project(tinyaudio)

add_library(${PROJECT_NAME} src/tinyaudio.cpp)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 11)
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

include(miniaudio.cmake)
target_link_libraries(${PROJECT_NAME} miniaudio)
