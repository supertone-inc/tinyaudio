#!/usr/bin/env bash

set -e

BUILD_DIR=./build

cmake -S. -B$BUILD_DIR -DCMAKE_BUILD_TYPE=Release -DTINYAUDIO_BUILD_TESTS=OFF
cmake --build $BUILD_DIR --config Release
