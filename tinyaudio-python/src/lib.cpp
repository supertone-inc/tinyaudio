#include <pybind11/functional.h>
#include <pybind11/numpy.h>
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <tinyaudio.cpp>

namespace py = pybind11;
using namespace pybind11::literals;
using namespace tinyaudio;

class TinyaudioPython : public Tinyaudio
{
public:
    using DataCallback = std::function<void(const py::array &input_frames, const py::array &output_frames)>;
    using StopCallback = tinyaudio::Tinyaudio::StopCallback;

    TinyaudioPython(
        bool offline,
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count,
        std::optional<std::string> input_file_path,
        std::optional<std::string> output_file_path,
        bool looping_input_file
    )
        : Tinyaudio(
              offline,
              format,
              channels,
              sample_rate,
              frame_count,
              input_file_path ? *input_file_path : "",
              output_file_path ? *output_file_path : "",
              looping_input_file
          )
        , bytes_per_sample(get_bytes_per_sample(get_format()))
        , type_code(
              [this]()
              {
                  switch (get_format())
                  {
                  case Format::U8:
                      return py::format_descriptor<uint8_t>::format();
                  case Format::S16:
                      return py::format_descriptor<int16_t>::format();
                  case Format::S32:
                      return py::format_descriptor<int32_t>::format();
                  case Format::F32:
                      return py::format_descriptor<float>::format();
                  default:
                      throw Error("unsupported format");
                  }
              }()
          )
        , sample_count(get_channels() * frame_count)
    {
    }

    void start(const DataCallback &data_callback, const StopCallback &stop_callback = nullptr)
    {
        user_data_callback = data_callback;
        user_stop_callback = stop_callback;

        Tinyaudio::start(
            std::bind(
                &TinyaudioPython::data_callback,
                this,
                std::placeholders::_1,
                std::placeholders::_2,
                std::placeholders::_3
            ),
            std::bind(&TinyaudioPython::stop_callback, this)
        );
    }

private:
    size_t bytes_per_sample;
    std::string type_code;
    size_t sample_count;

    DataCallback user_data_callback;
    StopCallback user_stop_callback;

    void data_callback(const void *input_frames, void *output_frames, size_t frame_count)
    {
        py::gil_scoped_acquire acquire;

        user_data_callback(
            py::array(
                py::buffer_info(
                    const_cast<void *>(input_frames),
                    bytes_per_sample,
                    type_code,
                    1,
                    {sample_count},
                    {bytes_per_sample},
                    true
                ),
                py::none()
            ),
            py::array(
                py::buffer_info(
                    output_frames,
                    bytes_per_sample,
                    type_code,
                    1,
                    {sample_count},
                    {bytes_per_sample},
                    false
                ),
                py::none()
            )
        );
    }

    void stop_callback()
    {
        if (!user_stop_callback)
        {
            return;
        }

        py::gil_scoped_acquire acquire;

        user_stop_callback();
    }
};

PYBIND11_MODULE(tinyaudio, m)
{
    py::enum_<Format>(m, "Format")
        .value("UNKNOWN", Format::UNKNOWN)
        .value("U8", Format::U8)
        .value("S16", Format::S16)
        .value("S24", Format::S24)
        .value("S32", Format::S32)
        .value("F32", Format::F32)
        .export_values();

    py::class_<TinyaudioPython>(m, "Tinyaudio")
        .def(
            py::init<
                bool,
                Format,
                size_t,
                size_t,
                size_t,
                std::optional<std::string>,
                std::optional<std::string>,
                bool>(),
            "offline"_a,
            "format"_a,
            "channels"_a,
            "sample_rate"_a,
            "frame_count"_a,
            "input_file_path"_a,
            "output_file_path"_a,
            "looping_input_file"_a
        )
        .def_property_readonly("offline", &TinyaudioPython::is_offline)
        .def_property_readonly("format", &TinyaudioPython::get_format)
        .def_property_readonly("channels", &TinyaudioPython::get_channels)
        .def_property_readonly("sample_rate", &TinyaudioPython::get_sample_rate)
        .def_property_readonly("frame_count", &TinyaudioPython::get_frame_count)
        .def_property(
            "looping_input_file",
            &TinyaudioPython::is_looping_input_file,
            &TinyaudioPython::set_looping_input_file
        )
        .def_property_readonly("started", &TinyaudioPython::is_started)
        .def("start", &TinyaudioPython::start, "data_callback"_a, "stop_callback"_a = nullptr)
        .def("stop", &TinyaudioPython::stop);

    py::class_<AudioFileInfo>(m, "AudioFileInfo")
        .def_readwrite("format", &AudioFileInfo::format)
        .def_readwrite("channels", &AudioFileInfo::channels)
        .def_readwrite("sample_rate", &AudioFileInfo::sample_rate)
        .def_readwrite("total_frame_count", &AudioFileInfo::total_frame_count);

    m.def("get_audio_file_info", &get_audio_file_info, "path"_a);
}
