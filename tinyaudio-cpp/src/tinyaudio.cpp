#include "codec_stream.hpp"
#include "device_stream.hpp"

#include <tinyaudio.hpp>

namespace tinyaudio
{
class Tinyaudio::Impl
{
public:
    Impl(
        bool offline,
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count,
        std::optional<std::string> input_file_path,
        std::optional<std::string> output_file_path,
        bool looping_input_file
    )
        : stream(nullptr)
    {
        if (offline)
        {
            stream = new CodecStream(
                *input_file_path,
                *output_file_path,
                EncodingFormat::WAV,
                format,
                channels,
                sample_rate,
                frame_count
            );
        }
        else
        {
            stream = new DeviceStream(
                format,
                channels,
                sample_rate,
                frame_count,
                input_file_path,
                output_file_path,
                looping_input_file
            );
        }
    }

    virtual ~Impl()
    {
        if (stream != nullptr)
        {
            delete stream;
            stream = nullptr;
        }
    }

    Stream *stream;
};

Tinyaudio::Tinyaudio(
    bool offline,
    Format format,
    size_t channels,
    size_t sample_rate,
    size_t frame_count,
    std::optional<std::string> input_file_path,
    std::optional<std::string> output_file_path,
    bool looping_input_file
)
    : impl(new Impl(
          offline,
          format,
          channels,
          sample_rate,
          frame_count,
          input_file_path,
          output_file_path,
          looping_input_file
      ))
{
}

Tinyaudio::Tinyaudio(Tinyaudio &&other)
    : impl(nullptr)
{
    *this = std::move(other);
}

Tinyaudio &Tinyaudio::operator=(Tinyaudio &&other)
{
    if (&other != this)
    {
        delete impl;
        impl = other.impl;
        other.impl = nullptr;
    }

    return *this;
}

Tinyaudio::~Tinyaudio()
{
    if (impl != nullptr)
    {
        delete impl;
        impl = nullptr;
    }
}

bool Tinyaudio::is_offline() const
{
    return dynamic_cast<CodecStream *>(impl->stream);
}

Format Tinyaudio::get_format() const
{
    return impl->stream->get_format();
}

size_t Tinyaudio::get_channels() const
{
    return impl->stream->get_channels();
}

size_t Tinyaudio::get_sample_rate() const
{
    return impl->stream->get_sample_rate();
}

size_t Tinyaudio::get_frame_count() const
{
    return impl->stream->get_frame_count();
}

bool Tinyaudio::is_looping_input_file() const
{
    auto device_stream = dynamic_cast<DeviceStream *>(impl->stream);

    if (device_stream != nullptr)
    {
        return device_stream->is_looping_input_file();
    }

    return false;
}

void Tinyaudio::set_looping_input_file(bool value)
{
    auto device_stream = dynamic_cast<DeviceStream *>(impl->stream);

    if (device_stream != nullptr)
    {
        device_stream->set_looping_input_file(value);
    }
}

bool Tinyaudio::is_started() const
{
    return impl->stream->is_started();
}

void Tinyaudio::start(const DataCallback &callback)
{
    impl->stream->start(callback);
}

void Tinyaudio::stop()
{
    impl->stream->stop();
}
} // namespace tinyaudio

#include <algorithm>
#include <chrono>
#include <doctest.h>
#include <thread>

namespace tinyaudio::tests::tinyaudio
{
const std::string INPUT_FILE_PATH = "../audio-samples/2MB.wav";
const Format FORMAT = Format::F32;
const size_t CHANNELS = 2;
const size_t SAMPLE_RATE = 44100;
const size_t FRAME_COUNT = 128;

TEST_CASE("[tinyaudio] works offline")
{
    Tinyaudio audio(
        true,
        FORMAT,
        CHANNELS,
        SAMPLE_RATE,
        FRAME_COUNT,
        "../audio-samples/2MB.wav",
        "test-tinyaudio-offline.wav",
        true
    );

    REQUIRE_EQ(audio.is_offline(), true);
    REQUIRE_EQ(audio.get_format(), FORMAT);
    REQUIRE_EQ(audio.get_channels(), CHANNELS);
    REQUIRE_EQ(audio.get_sample_rate(), SAMPLE_RATE);
    REQUIRE_EQ(audio.get_frame_count(), FRAME_COUNT);
    REQUIRE_EQ(audio.is_looping_input_file(), false);
    REQUIRE_EQ(audio.is_started(), false);

    audio.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            REQUIRE_EQ(audio.is_started(), true);
            std::copy_n(
                static_cast<const float *>(input_frames),
                audio.get_channels() * frame_count,
                static_cast<float *>(output_frames)
            );
        }
    );

    audio.stop();
    REQUIRE_EQ(audio.is_started(), false);
}

TEST_CASE("[tinyaudio] works online")
{
    Tinyaudio audio(
        false,
        FORMAT,
        CHANNELS,
        SAMPLE_RATE,
        FRAME_COUNT,
        "../audio-samples/2MB.wav",
        "test-tinyaudio-online.wav",
        false
    );

    REQUIRE_EQ(audio.is_offline(), false);
    REQUIRE_EQ(audio.get_format(), FORMAT);
    REQUIRE_EQ(audio.get_channels(), CHANNELS);
    REQUIRE_EQ(audio.get_sample_rate(), SAMPLE_RATE);
    REQUIRE_EQ(audio.get_frame_count(), FRAME_COUNT);
    REQUIRE_EQ(audio.is_looping_input_file(), false);
    REQUIRE_EQ(audio.is_started(), false);

    audio.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            std::copy_n(
                static_cast<const float *>(input_frames),
                audio.get_channels() * frame_count,
                static_cast<float *>(output_frames)
            );
        }
    );
    REQUIRE_EQ(audio.is_started(), true);

    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    audio.stop();
    REQUIRE_EQ(audio.is_started(), false);
}
} // namespace tinyaudio::tests::tinyaudio
