#!/usr/bin/env bash

set -e

BUILD_DIR=build

if [[ -n $DEBUG ]]; then
    CMAKE_BUILD_TYPE=Debug

    cmake -S. -B$BUILD_DIR -DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE -DTINYAUDIO_BUILD_TESTS=ON
    cmake --build $BUILD_DIR --config $CMAKE_BUILD_TYPE
    cmake --install $BUILD_DIR --config $CMAKE_BUILD_TYPE --prefix $BUILD_DIR

    exit 0
fi

CMAKE_BUILD_TYPE=Release

cmake -S. -B$BUILD_DIR -DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE -DTINYAUDIO_BUILD_TESTS=OFF
cmake --build $BUILD_DIR --config $CMAKE_BUILD_TYPE
