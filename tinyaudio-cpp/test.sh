#!/usr/bin/env bash

set -e

BUILD_DIR=build

cmake -S. -B$BUILD_DIR -DCMAKE_BUILD_TYPE=Debug -DTINYAUDIO_BUILD_TESTS=ON
cmake --build $BUILD_DIR --config Debug
cmake --install $BUILD_DIR --config Debug --prefix $BUILD_DIR
./$BUILD_DIR/bin/tinyaudio-test
