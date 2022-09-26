from pybind11.setup_helpers import Pybind11Extension
from setuptools import setup
import os

os.system("git submodule update --init --recursive ../miniaudio")

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
