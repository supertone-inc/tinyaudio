#pragma once

#include "common.hpp"

#include <algorithm>
#include <string>
#include <vector>

namespace tinyaudio
{
class Decoder
{
public:
    Decoder(const std::string &input_file_path)
        : Decoder(input_file_path, Format::UNKNOWN, 0, 0)
    {
    }

    Decoder(const std::string &input_file_path, Format output_format, size_t output_channels, size_t output_sample_rate)
    {
        auto config =
            ma_decoder_config_init(static_cast<ma_format>(output_format), output_channels, output_sample_rate);

        check_result(ma_decoder_init_file(input_file_path.c_str(), &config, &raw_decoder));
        check_result(ma_decoder_get_length_in_pcm_frames(&raw_decoder, &total_frame_count));
    }

    Decoder(const std::wstring &input_file_path)
        : Decoder(input_file_path, Format::UNKNOWN, 0, 0)
    {
    }

    Decoder(
        const std::wstring &input_file_path,
        Format output_format,
        size_t output_channels,
        size_t output_sample_rate
    )
    {
        auto config =
            ma_decoder_config_init(static_cast<ma_format>(output_format), output_channels, output_sample_rate);

        check_result(ma_decoder_init_file_w(input_file_path.c_str(), &config, &raw_decoder));
        check_result(ma_decoder_get_length_in_pcm_frames(&raw_decoder, &total_frame_count));
    }

    virtual ~Decoder()
    {
        close();
    }

    Format get_format() const
    {
        return static_cast<Format>(raw_decoder.outputFormat);
    }

    size_t get_channels() const
    {
        return raw_decoder.outputChannels;
    }

    size_t get_sample_rate() const
    {
        return raw_decoder.outputSampleRate;
    }

    size_t get_total_frame_count() const
    {
        return total_frame_count;
    }

    size_t get_available_frame_count()
    {
        ma_uint64 value = 0;
        check_result(ma_decoder_get_available_frames(&raw_decoder, &value));
        return value;
    }

    bool is_looping() const
    {
        return ma_data_source_is_looping(&raw_decoder);
    }

    void set_looping(bool value)
    {
        check_result(ma_data_source_set_looping(&raw_decoder, value));
    }

    void seek(size_t frame_index)
    {
        check_result(ma_decoder_seek_to_pcm_frame(&raw_decoder, frame_index));
    }

    size_t read(void *frames, size_t frame_count)
    {
        auto byte_count = get_bytes_per_frame(get_format(), get_channels()) * frame_count;
        std::fill_n(static_cast<uint8_t *>(frames), byte_count, 0);

        ma_uint64 frames_read = 0;
        auto result = ma_data_source_read_pcm_frames(&raw_decoder, frames, frame_count, &frames_read);
        switch (result)
        {
        case MA_SUCCESS:
        case MA_AT_END:
            break;
        default:
            check_result(result);
        }

        return frames_read;
    }

    void close()
    {
        if (raw_decoder.data.vfs.file != nullptr)
        {
            ma_decoder_uninit(&raw_decoder);
        }
    }

private:
    ma_decoder raw_decoder;
    ma_uint64 total_frame_count;
};

namespace tests::decoder
{
const std::string INPUT_FILE_PATH = "../audio-samples/1MB.wav";
const size_t FRAME_COUNT = 128;

bool check_frames_zero_padded(const uint8_t *frame_bytes, size_t byte_count, size_t non_zero_byte_count)
{
    for (auto i = non_zero_byte_count; i < byte_count; i++)
    {
        if (frame_bytes[i] != 0)
        {
            return false;
        }
    }

    return true;
}

TEST_CASE("[decoder] returns correct metadata")
{
    SUBCASE("without config")
    {
        Decoder decoder(INPUT_FILE_PATH);

        REQUIRE_NE(decoder.get_format(), Format::UNKNOWN);
        REQUIRE_GT(decoder.get_channels(), 0);
        REQUIRE_GT(decoder.get_sample_rate(), 0);
        REQUIRE_GT(decoder.get_total_frame_count(), 0);
        REQUIRE_EQ(decoder.get_available_frame_count(), decoder.get_total_frame_count());
    }

    SUBCASE("with config")
    {
        Decoder decoder(INPUT_FILE_PATH, Format::F32, 1, 44100);

        REQUIRE_EQ(decoder.get_format(), Format::F32);
        REQUIRE_EQ(decoder.get_channels(), 1);
        REQUIRE_EQ(decoder.get_sample_rate(), 44100);
        REQUIRE_GT(decoder.get_total_frame_count(), 0);
        REQUIRE_EQ(decoder.get_available_frame_count(), decoder.get_total_frame_count());
    }
}

TEST_CASE("[decoder] seeks to frames")
{
    Decoder decoder(INPUT_FILE_PATH);

    decoder.seek(decoder.get_total_frame_count());
    REQUIRE_EQ(decoder.get_available_frame_count(), 0);

    decoder.seek(decoder.get_total_frame_count() / 2);
    REQUIRE_EQ(
        decoder.get_available_frame_count(),
        decoder.get_total_frame_count() - (decoder.get_total_frame_count() / 2)
    );

    decoder.seek(0);
    REQUIRE_EQ(decoder.get_available_frame_count(), decoder.get_total_frame_count());
}

TEST_CASE("[decoder] reads frames")
{
    SUBCASE("without config")
    {
        Decoder decoder(INPUT_FILE_PATH);

        auto bytes_per_frame = get_bytes_per_frame(decoder.get_format(), decoder.get_channels());
        std::vector<uint8_t> frames(bytes_per_frame * FRAME_COUNT);
        auto total_frames_read = 0;

        while (true)
        {
            auto frames_read = decoder.read(frames.data(), FRAME_COUNT);
            total_frames_read += frames_read;

            REQUIRE(check_frames_zero_padded(frames.data(), frames.size(), bytes_per_frame * frames_read));

            if (frames_read == 0)
            {
                break;
            }
        }

        REQUIRE_EQ(total_frames_read, decoder.get_total_frame_count());
    }

    SUBCASE("with config")
    {
        Decoder decoder(INPUT_FILE_PATH, Format::S16, 1, 44100);

        auto bytes_per_frame = get_bytes_per_frame(decoder.get_format(), decoder.get_channels());
        std::vector<uint8_t> frames(bytes_per_frame * FRAME_COUNT);
        auto total_frames_read = 0;

        while (true)
        {
            size_t frames_read = decoder.read(frames.data(), FRAME_COUNT);
            total_frames_read += frames_read;

            REQUIRE(check_frames_zero_padded(frames.data(), frames.size(), bytes_per_frame * frames_read));

            if (frames_read == 0)
            {
                break;
            }
        }

        REQUIRE(total_frames_read + FRAME_COUNT > decoder.get_total_frame_count());
    }
}

TEST_CASE("[decoder] loops")
{
    Decoder decoder(INPUT_FILE_PATH);

    REQUIRE_EQ(decoder.is_looping(), false);
    decoder.set_looping(true);
    REQUIRE_EQ(decoder.is_looping(), true);

    auto bytes_per_frame = get_bytes_per_frame(decoder.get_format(), decoder.get_channels());
    std::vector<uint8_t> frames(bytes_per_frame * FRAME_COUNT);
    auto total_frames_read = 0;

    while (true)
    {
        size_t frames_read = decoder.read(frames.data(), FRAME_COUNT);
        total_frames_read += frames_read;

        REQUIRE(check_frames_zero_padded(frames.data(), frames.size(), bytes_per_frame * frames_read));

        if (frames_read == 0)
        {
            break;
        }

        if (total_frames_read > decoder.get_total_frame_count())
        {
            decoder.set_looping(false);
        }
    }

    REQUIRE_EQ(total_frames_read, 2 * decoder.get_total_frame_count());
}

TEST_CASE("[decoder] closes without error")
{
    Decoder decoder(INPUT_FILE_PATH);
    decoder.close();
}
} // namespace tests::decoder
} // namespace tinyaudio
