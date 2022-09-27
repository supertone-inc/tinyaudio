#include "codec_stream.hpp"
#include "device_stream.hpp"

#include <optional>
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
    const std::string &input_file_path,
    const std::string &output_file_path,
    bool looping_input_file
)
    : impl(new Impl(
          offline,
          format,
          channels,
          sample_rate,
          frame_count,
          input_file_path.empty() ? std::nullopt : std::optional(input_file_path),
          output_file_path.empty() ? std::nullopt : std::optional(output_file_path),
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

void Tinyaudio::start(const DataCallback &data_callback, const StopCallback &stop_callback)
{
    impl->stream->start(data_callback, stop_callback);
}

void Tinyaudio::stop()
{
    impl->stream->stop();
}

AudioFileInfo get_audio_file_info(const std::string &path)
{
    Decoder decoder(path, Format::UNKNOWN, 0, 0, false);
    return AudioFileInfo{
        decoder.get_format(),
        decoder.get_channels(),
        decoder.get_sample_rate(),
        decoder.get_total_frame_count()};
}
} // namespace tinyaudio

#ifdef TINYAUDIO_BUILD_TESTS
#include "common.hpp"

#include <algorithm>

namespace tinyaudio::tests::tinyaudio
{
const auto FORMAT = Format::F32;
const auto CHANNELS = 2;
const auto SAMPLE_RATE = 44100;
const auto FRAME_COUNT = 128;

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
                reinterpret_cast<const float *>(input_frames),
                audio.get_channels() * frame_count,
                reinterpret_cast<float *>(output_frames)
            );
        },
        [&]() { REQUIRE_EQ(audio.is_started(), false); }
    );

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
                reinterpret_cast<const float *>(input_frames),
                audio.get_channels() * frame_count,
                reinterpret_cast<float *>(output_frames)
            );
            notify();
        },
        [&]() { REQUIRE_EQ(audio.is_started(), false); }
    );
    REQUIRE_EQ(audio.is_started(), true);

    wait();

    audio.stop();
    REQUIRE_EQ(audio.is_started(), false);
}

TEST_CASE("[tinyaudio] get_audio_file_info() retrives metadata correctly")
{
    auto info = get_audio_file_info("../audio-samples/1MB.wav");

    REQUIRE_EQ(info.format, Format::S16);
    REQUIRE_EQ(info.channels, 2);
    REQUIRE_EQ(info.sample_rate, 8000);
    REQUIRE_EQ(info.total_frame_count, 268237);
}
} // namespace tinyaudio::tests::tinyaudio
#endif
