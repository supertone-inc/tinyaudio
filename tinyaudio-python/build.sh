#!/usr/bin/env bash

set -e

if [[ -n $DEBUG ]]; then
    CPPFLAGS="-g" python -m pip install --target . --upgrade --no-clean .
    exit 0
fi

python -m pip install --target . --upgrade --no-deps .
