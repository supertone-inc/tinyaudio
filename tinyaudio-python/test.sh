#!/usr/bin/env bash

set -e

pip install --target . --upgrade .
python test.py
