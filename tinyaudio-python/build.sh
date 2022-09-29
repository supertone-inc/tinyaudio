#!/usr/bin/env bash

set -e

if [[ -n $DEBUG ]]; then
    CPPFLAGS="-g" pip install --target . --upgrade --no-clean .
    exit 0
fi

pip install --target . --upgrade --no-deps .
