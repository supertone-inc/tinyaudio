#include <pybind11/functional.h>
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <tinyaudio.cpp>

namespace py = pybind11;

using namespace tinyaudio;

class TinyaudioPython : public Tinyaudio
{
public:
    using DataCallback = std::function<void(py::memoryview input_frames, py::memoryview output_frames)>;
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
    {
    }

    void start(const DataCallback &data_callback, const StopCallback &stop_callback = nullptr)
    {
        user_data_callback = std::move(data_callback);
        Tinyaudio::start(
            std::bind(
                &TinyaudioPython::data_callback,
                this,
                std::placeholders::_1,
                std::placeholders::_2,
                std::placeholders::_3
            ),
            stop_callback
        );
    }

private:
    DataCallback user_data_callback;

    void data_callback(const void *input_frames, void *output_frames, size_t frame_count)
    {
        auto bytes_per_sample = get_bytes_per_sample(get_format());

        std::string type_code;
        switch (get_format())
        {
        case Format::U8:
            type_code = py::format_descriptor<uint8_t>::format();
            break;
        case Format::S16:
            type_code = py::format_descriptor<int16_t>::format();
            break;
        case Format::S32:
            type_code = py::format_descriptor<int32_t>::format();
            break;
        case Format::F32:
            type_code = py::format_descriptor<float>::format();
            break;
        default:
            throw Error("unsupported format");
        }

        auto channels = get_channels();

        user_data_callback(
            py::memoryview::from_buffer(
                (void *)input_frames,
                bytes_per_sample,
                type_code.c_str(),
                {channels * frame_count},
                {bytes_per_sample},
                true
            ),
            py::memoryview::from_buffer(
                output_frames,
                bytes_per_sample,
                type_code.c_str(),
                {channels * frame_count},
                {bytes_per_sample},
                false
            )
        );
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
        .def(py::init<
             bool,
             Format,
             size_t,
             size_t,
             size_t,
             std::optional<std::string>,
             std::optional<std::string>,
             bool>())
        .def("is_offline", &TinyaudioPython::is_offline)
        .def("get_format", &TinyaudioPython::get_format)
        .def("get_channels", &TinyaudioPython::get_channels)
        .def("get_sample_rate", &TinyaudioPython::get_sample_rate)
        .def("get_frame_count", &TinyaudioPython::get_frame_count)
        .def("is_looping_input_file", &TinyaudioPython::is_looping_input_file)
        .def("set_looping_input_file", &TinyaudioPython::set_looping_input_file)
        .def("is_started", &TinyaudioPython::is_started)
        .def("start", &TinyaudioPython::start)
        .def("stop", &TinyaudioPython::stop);

    py::class_<AudioFileInfo>(m, "AudioFileInfo")
        .def_readwrite("format", &AudioFileInfo::format)
        .def_readwrite("channels", &AudioFileInfo::channels)
        .def_readwrite("sample_rate", &AudioFileInfo::sample_rate)
        .def_readwrite("total_frame_count", &AudioFileInfo::total_frame_count);

    m.def("get_audio_file_info", &get_audio_file_info);
}