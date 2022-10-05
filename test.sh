#!/usr/bin/env bash

set -e

(
    cd tinyaudio-cpp
    ./test.sh
)

(
    cd tinyaudio-python
    ./test.sh
)
