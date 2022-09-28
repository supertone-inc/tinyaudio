from setuptools import setup
import os
import sys

os.system("git submodule update --init --recursive ../miniaudio pybind11")

PROJECT_DIR = os.path.abspath(os.path.dirname(__file__))
sys.path.append(os.path.join(PROJECT_DIR, "pybind11"))
from pybind11.setup_helpers import Pybind11Extension

ext_modules = [
    Pybind11Extension(
        "tinyaudio",
        ["src/lib.cpp"],
        cxx_std=17,
        include_dirs=[
            "../miniaudio",
            "../tinyaudio-cpp/include",
            "../tinyaudio-cpp/src",
        ],
    )
]

setup(
    name="tinyaudio",
    ext_modules=ext_modules,
)
