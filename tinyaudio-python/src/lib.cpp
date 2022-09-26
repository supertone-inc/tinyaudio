#include <pybind11/pybind11.h>
#include <tinyaudio.cpp>

namespace py = pybind11;
using namespace tinyaudio;

PYBIND11_MODULE(tinyaudio, m)
{
    py::class_<Tinyaudio>(m, "Tinyaudio");
}
