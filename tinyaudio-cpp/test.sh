#!/usr/bin/env bash

set -e

BUILD_DIR=./build

cmake -S. -B$BUILD_DIR -DTINYAUDIO_BUILD_TESTS=ON
cmake --build $BUILD_DIR
ctest --test-dir $BUILD_DIR -V --no-tests=error
