include(FetchContent)

FetchContent_Declare(
    doctest
    GIT_REPOSITORY https://github.com/doctest/doctest.git
    GIT_TAG v2.4.9
    GIT_SHALLOW TRUE
)

FetchContent_GetProperties(doctest)

if(NOT doctest_POPULATED)
    FetchContent_Populate(doctest)
    add_library(doctest INTERFACE)
    target_include_directories(doctest INTERFACE ${doctest_SOURCE_DIR}/doctest)
    include(${doctest_SOURCE_DIR}/scripts/cmake/doctest.cmake)
endif()
