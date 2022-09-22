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

    void start(const DataCallback &callback) override
    {
        data_callback = std::move(callback);
        device.start(this, device_data_callback);
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

    static void device_data_callback(
        void *user_data,
        const void *nullable_input_frames,
        void *output_frames,
        size_t frame_count
    )
    {
        auto &self = *static_cast<DeviceStream *>(user_data);

        auto input_frames = nullable_input_frames;

        if (self.decoder)
        {
            auto decoder_frames = (*self.decoder_buffer).data();
            auto frames_read = (*self.decoder).read(decoder_frames, frame_count);
            input_frames = decoder_frames;

            if (frames_read == 0)
            {
                self.stop();
            }
        }

        self.data_callback(input_frames, output_frames, frame_count);

        if (self.encoder)
        {
            (*self.encoder).write(output_frames, frame_count);
        }
    }
};
} // namespace tinyaudio

#include <algorithm>
#include <vector>

namespace tinyaudio::tests::device_stream
{
const std::string INPUT_FILE_PATH = "../audio-samples/2MB.wav";
const std::string OUTPUT_FILE_PATH = "test-device-stream.wav";
const Format FORMAT = Format::F32;
const size_t CHANNELS = 2;
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
    REQUIRE_EQ(stream.is_started(), false);

    stream.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            std::copy_n(
                static_cast<const float *>(input_frames),
                stream.get_channels() * frame_count,
                static_cast<float *>(output_frames)
            );
        }
    );
    REQUIRE_EQ(stream.is_started(), true);

    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    stream.stop();
    REQUIRE_EQ(stream.is_started(), false);
}

} // namespace tinyaudio::tests::device_stream
