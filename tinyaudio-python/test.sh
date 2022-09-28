#!/usr/bin/env bash

set -e

export CPPFLAGS="-g"

pip install --target . --upgrade --no-clean .

if [[ -z $NO_RUN ]]; then
    python test.py
fi
