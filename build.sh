#!/usr/bin/env bash

set -e

(
    cd tinyaudio-cpp
    ./build.sh
)

(
    cd tinyaudio-python
    ./build.sh
)
