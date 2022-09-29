#!/usr/bin/env bash

set -e

DEBUG=1 ./build.sh
./build/bin/tinyaudio-test $@
