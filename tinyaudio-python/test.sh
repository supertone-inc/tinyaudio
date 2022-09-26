#!/usr/bin/env bash

set -e

pip install --target tinyaudio --upgrade .
PYTHONPATH=tinyaudio python tests/test.py
