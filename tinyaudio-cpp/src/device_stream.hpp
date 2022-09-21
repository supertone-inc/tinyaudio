#pragma once

#include "common.hpp"
#include "decoder.hpp"
#include "device.hpp"
#include "encoder.hpp"
#include "stream.hpp"

#include <algorithm>
#include <functional>
#include <optional>
#include <string>
#include <vector>

namespace tinyaudio
{
class DeviceStream : public Stream
{
public:
    DeviceStream(
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count,
        std::optional<std::string> input_file_path,
        std::optional<std::string> output_file_path,
        bool looping_input_file
    )
        : device(
              input_file_path ? DeviceType::PLAYBACK : DeviceType::DUPLEX,
              format,
              channels,
              sample_rate,
              frame_count
          )
    {
        if (input_file_path)
        {
            decoder.emplace(*input_file_path, format, channels, sample_rate, looping_input_file);
            decoder_buffer.emplace(get_bytes_per_frame(format, channels) * frame_count);
        }

        if (output_file_path)
        {
            encoder.emplace(*output_file_path, EncodingFormat::WAV, format, channels, sample_rate);
        }
    }

    DeviceStream(
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count,
        std::optional<std::wstring> input_file_path,
        std::optional<std::wstring> output_file_path,
        bool looping_input_file
    )
        : device(
              input_file_path ? DeviceType::PLAYBACK : DeviceType::DUPLEX,
              format,
              channels,
              sample_rate,
              frame_count
          )
    {
        if (input_file_path)
        {
            decoder.emplace(*input_file_path, format, channels, sample_rate, looping_input_file);
            decoder_buffer.emplace(get_bytes_per_frame(format, channels) * frame_count);
        }

        if (output_file_path)
        {
            encoder.emplace(*output_file_path, EncodingFormat::WAV, format, channels, sample_rate);
        }
    }

    DeviceType get_device_type() const
    {
        return device.get_device_type();
    }

    Format get_format() const
    {
        return device.get_format();
    }

    size_t get_channels() const
    {
        return device.get_channels();
    }

    size_t get_sample_rate() const
    {
        return device.get_sample_rate();
    }

    size_t get_frame_count() const
    {
        return device.get_frame_count();
    }

    bool is_looping_input_file() const
    {
        return decoder ? (*decoder).is_looping() : false;
    }

    void set_loopping_input_file(bool value)
    {
        if (decoder)
        {
            (*decoder).set_looping(value);
        }
    }

    DeviceState get_device_state() const
    {
        return device.get_device_state();
    }

    bool is_started() const
    {
        return device.is_started();
    }

    void start(const DataCallback &callback) override
    {
        device.start(
            [&](auto nullable_input_frames, auto output_frames, auto frame_count)
            {
                auto input_frames = nullable_input_frames;

                if (decoder)
                {
                    auto decoder_frames = (*decoder_buffer).data();
                    auto frames_read = (*decoder).read(decoder_frames, frame_count);
                    input_frames = decoder_frames;

                    if (frames_read == 0)
                    {
                        stop();
                    }
                }

                callback(input_frames, output_frames, frame_count);

                if (encoder)
                {
                    (*encoder).write(output_frames, frame_count);
                }
            }
        );
    }

    void stop() override
    {
        device.stop();
    }

private:
    Device device;
    std::optional<Decoder> decoder;
    std::optional<std::vector<uint8_t>> decoder_buffer;
    std::optional<Encoder> encoder;
};

namespace tests::device_stream
{
const std::string INPUT_FILE_PATH = "../audio-samples/2MB.wav";
const std::string OUTPUT_FILE_PATH = "test-device-stream.wav";
const Format FORMAT = Format::F32;
const size_t CHANNELS = 1;
const size_t SAMPLE_RATE = 44100;
const size_t FRAME_COUNT = 128;

TEST_CASE("[device_stream] works")
{
    DeviceStream stream(FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT, INPUT_FILE_PATH, OUTPUT_FILE_PATH, false);

    REQUIRE_EQ(stream.get_device_type(), DeviceType::PLAYBACK);
    REQUIRE_EQ(stream.get_format(), FORMAT);
    REQUIRE_EQ(stream.get_channels(), CHANNELS);
    REQUIRE_EQ(stream.get_sample_rate(), SAMPLE_RATE);
    REQUIRE_EQ(stream.get_frame_count(), FRAME_COUNT);
    REQUIRE_EQ(stream.is_looping_input_file(), false);

    stream.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        { std::copy_n(static_cast<const float *>(input_frames), frame_count, static_cast<float *>(output_frames)); }
    );

    std::this_thread::sleep_for(std::chrono::milliseconds(100));
}

} // namespace tests::device_stream
} // namespace tinyaudio
