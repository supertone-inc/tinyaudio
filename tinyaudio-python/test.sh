#!/usr/bin/env bash

set -e

DEBUG=1 ./build.sh
python test.py $@
