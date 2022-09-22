#!/usr/bin/env bash

set -e

BUILD_DIR=build
CMAKE_BUILD_TYPE=Debug

cmake -S. -B$BUILD_DIR -DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE -DTINYAUDIO_BUILD_TESTS=ON
cmake --build $BUILD_DIR --config $CMAKE_BUILD_TYPE
cmake --install $BUILD_DIR --config $CMAKE_BUILD_TYPE --prefix $BUILD_DIR

if [[ -z $NO_RUN ]]; then
    ./$BUILD_DIR/bin/tinyaudio-test
fi
