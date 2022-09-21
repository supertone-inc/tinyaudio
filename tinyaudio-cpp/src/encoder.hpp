#pragma once

#include "common.hpp"

#include <string>
#include <vector>

namespace tinyaudio
{
enum class EncodingFormat
{
    WAV = ma_encoding_format_wav
};

class Encoder
{
public:
    Encoder(
        const std::string &output_file_path,
        EncodingFormat encoding_format,
        Format format,
        size_t channels,
        size_t sample_rate
    )
    {
        auto config = ma_encoder_config_init(
            static_cast<ma_encoding_format>(encoding_format),
            static_cast<ma_format>(format),
            channels,
            sample_rate
        );

        check_result(ma_encoder_init_file(output_file_path.c_str(), &config, &encoder));
    }

    Encoder(
        const std::wstring &output_file_path,
        EncodingFormat encoding_format,
        Format format,
        size_t channels,
        size_t sample_rate
    )
    {
        auto config = ma_encoder_config_init(
            static_cast<ma_encoding_format>(encoding_format),
            static_cast<ma_format>(format),
            channels,
            sample_rate
        );

        check_result(ma_encoder_init_file_w(output_file_path.c_str(), &config, &encoder));
    }

    virtual ~Encoder()
    {
        close();
    }

    EncodingFormat get_encoding_format() const
    {
        return static_cast<EncodingFormat>(encoder.config.encodingFormat);
    }

    Format get_format() const
    {
        return static_cast<Format>(encoder.config.format);
    }

    size_t get_channels() const
    {
        return encoder.config.channels;
    }

    size_t get_sample_rate() const
    {
        return encoder.config.sampleRate;
    }

    size_t write(const void *frames, size_t frame_count)
    {
        ma_uint64 frames_written = 0;
        check_result(ma_encoder_write_pcm_frames(&encoder, frames, frame_count, &frames_written));
        return frames_written;
    }

    void close()
    {
        if (encoder.data.vfs.file != nullptr)
        {
            ma_encoder_uninit(&encoder);
        }
    }

private:
    ma_encoder encoder;
};

namespace tests::encoder
{
const std::string OUTPUT_FILE_PATH = "test-encoder.wav";
const std::string OUTPUT_FILE_PATH_NO_SOUND = "test-encoder-no-sound.wav";
const EncodingFormat ENCODING_FORMAT = EncodingFormat::WAV;
const Format FORMAT = Format::F32;
const size_t CHANNELS = 2;
const size_t SAMPLE_RATE = 44100;
const size_t FRAME_COUNT = 128;
const size_t DURATION_IN_SECS = 1;
const size_t LOOP_COUNT = DURATION_IN_SECS * SAMPLE_RATE / FRAME_COUNT;

TEST_CASE("[encoder] retrives metadata")
{
    Encoder encoder(OUTPUT_FILE_PATH_NO_SOUND, ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);

    REQUIRE_EQ(encoder.get_encoding_format(), ENCODING_FORMAT);
    REQUIRE_EQ(encoder.get_format(), FORMAT);
    REQUIRE_EQ(encoder.get_channels(), CHANNELS);
    REQUIRE_EQ(encoder.get_sample_rate(), SAMPLE_RATE);
}

TEST_CASE("[encoder] writes frames")
{
    Encoder encoder(OUTPUT_FILE_PATH, ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);

    auto waveform_config =
        ma_waveform_config_init(static_cast<ma_format>(FORMAT), CHANNELS, SAMPLE_RATE, ma_waveform_type_sine, 0.5, 440);
    ma_waveform waveform;
    check_result(ma_waveform_init(&waveform_config, &waveform));

    auto bytes_per_frame = get_bytes_per_frame(encoder.get_format(), encoder.get_channels());
    std::vector<uint8_t> frames(bytes_per_frame * FRAME_COUNT);
    auto total_frames_written = 0;

    CHECK_NOTHROW((
        [&]()
        {
            for (auto i = 0; i < LOOP_COUNT; i++)
            {
                check_result(ma_waveform_read_pcm_frames(&waveform, frames.data(), FRAME_COUNT, nullptr));
                total_frames_written += encoder.write(frames.data(), FRAME_COUNT);
            }
        }
    )());

    CHECK_EQ(total_frames_written, LOOP_COUNT * FRAME_COUNT);

    ma_waveform_uninit(&waveform);
}

TEST_CASE("[encoder] closes without error")
{
    Encoder encoder(OUTPUT_FILE_PATH_NO_SOUND, ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);
    encoder.close();
}
} // namespace tests::encoder
} // namespace tinyaudio
