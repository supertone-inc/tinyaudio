#pragma once

#include "common.hpp"
#include "decoder.hpp"
#include "device.hpp"
#include "encoder.hpp"
#include "stream.hpp"

#include <functional>
#include <optional>
#include <string>

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
        std::optional<std::variant<std::string, std::wstring>> input_file_path,
        std::optional<std::variant<std::string, std::wstring>> output_file_path,
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

    Format get_format() const override
    {
        return device.get_format();
    }

    size_t get_channels() const override
    {
        return device.get_channels();
    }

    size_t get_sample_rate() const override
    {
        return device.get_sample_rate();
    }

    size_t get_frame_count() const override
    {
        return device.get_frame_count();
    }

    bool is_looping_input_file() const
    {
        return decoder ? (*decoder).is_looping() : false;
    }

    void set_looping_input_file(bool value)
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

    bool is_started() const override
    {
        return device.is_started();
    }

    void start(const DataCallback &data_callback, const StopCallback &stop_callback = nullptr) override
    {
        this->data_callback = data_callback;
        this->stop_callback = stop_callback;

        device.start(
            this,
            std::bind(
                &DeviceStream::device_data_callback,
                this,
                std::placeholders::_1,
                std::placeholders::_2,
                std::placeholders::_3,
                std::placeholders::_4
            ),
            std::bind(&DeviceStream::device_stop_callback, this, std::placeholders::_1)
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
    DataCallback data_callback;
    StopCallback stop_callback;

    void device_data_callback(
        void *user_data,
        const void *nullable_input_frames,
        void *output_frames,
        size_t frame_count
    )
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

        data_callback(input_frames, output_frames, frame_count);

        if (encoder)
        {
            (*encoder).write(output_frames, frame_count);
        }
    }

    void device_stop_callback(void *user_data)
    {
        if (stop_callback)
        {
            stop_callback();
        }
    }
};
} // namespace tinyaudio

#ifdef TINYAUDIO_BUILD_TESTS
#include <algorithm>
#include <vector>

namespace tinyaudio::tests::device_stream
{
const auto INPUT_FILE_PATH = "../audio-samples/2MB.wav";
const auto OUTPUT_FILE_PATH = "test-device-stream.wav";
const auto FORMAT = Format::F32;
const auto CHANNELS = 2;
const auto SAMPLE_RATE = 44100;
const auto FRAME_COUNT = 128;

TEST_CASE("[device_stream] works")
{
    DeviceStream stream(FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT, INPUT_FILE_PATH, OUTPUT_FILE_PATH, false);

    REQUIRE_EQ(stream.get_device_type(), DeviceType::PLAYBACK);
    REQUIRE_EQ(stream.get_format(), FORMAT);
    REQUIRE_EQ(stream.get_channels(), CHANNELS);
    REQUIRE_EQ(stream.get_sample_rate(), SAMPLE_RATE);
    REQUIRE_EQ(stream.get_frame_count(), FRAME_COUNT);
    REQUIRE_EQ(stream.is_looping_input_file(), false);
    REQUIRE_EQ(stream.is_started(), false);

    stream.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            std::copy_n(
                reinterpret_cast<const float *>(input_frames),
                stream.get_channels() * frame_count,
                reinterpret_cast<float *>(output_frames)
            );
            notify();
        },
        [&]() { REQUIRE_EQ(stream.is_started(), false); }
    );
    REQUIRE_EQ(stream.is_started(), true);

    wait();

    stream.stop();
    REQUIRE_EQ(stream.is_started(), false);
}

} // namespace tinyaudio::tests::device_stream
#endif
