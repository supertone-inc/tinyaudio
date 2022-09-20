#pragma once

#include "error.hpp"
#include "format.hpp"

#include <doctest.h>
#include <miniaudio.h>
#include <string>
#include <tinyaudio.hpp>
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

        check_result(ma_decoder_init_file(input_file_path.c_str(), &config, &m_decoder));
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

        check_result(ma_decoder_init_file_w(input_file_path.c_str(), &config, &m_decoder));
    }

    Format format() const
    {
        return static_cast<Format>(m_decoder.outputFormat);
    }

    size_t channels() const
    {
        return m_decoder.outputChannels;
    }

    size_t sample_rate() const
    {
        return m_decoder.outputSampleRate;
    }

    size_t total_frame_count()
    {
        ma_uint64 frame_count = 0;
        check_result(ma_decoder_get_length_in_pcm_frames(&m_decoder, &frame_count));
        return frame_count;
    }

    size_t available_frame_count()
    {
        ma_uint64 frame_count = 0;
        check_result(ma_decoder_get_available_frames(&m_decoder, &frame_count));
        return frame_count;
    }

    void seek(size_t frame_index)
    {
        check_result(ma_decoder_seek_to_pcm_frame(&m_decoder, frame_index));
    }

    size_t read(void *frames, size_t frame_count)
    {
        ma_uint64 frames_read = 0;
        auto result = ma_decoder_read_pcm_frames(&m_decoder, frames, frame_count, &frames_read);
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
        if (m_decoder.data.vfs.file != nullptr)
        {
            ma_decoder_uninit(&m_decoder);
        }
    }

    virtual ~Decoder()
    {
        close();
    }

private:
    ma_decoder m_decoder;
};

TEST_CASE("[decoder] retrives metadata")
{
    SUBCASE("without config")
    {
        Decoder decoder("../audio-samples/700KB.mp3");

        REQUIRE_EQ(decoder.format(), Format::F32);
        REQUIRE_EQ(decoder.channels(), 2);
        REQUIRE_EQ(decoder.sample_rate(), 32000);
        REQUIRE_EQ(decoder.total_frame_count(), 873216);
        REQUIRE_EQ(decoder.available_frame_count(), decoder.total_frame_count());
    }

    SUBCASE("with config")
    {
        Decoder decoder("../audio-samples/700KB.mp3", Format::S16, 1, 44100);

        REQUIRE_EQ(decoder.format(), Format::S16);
        REQUIRE_EQ(decoder.channels(), 1);
        REQUIRE_EQ(decoder.sample_rate(), 44100);
        REQUIRE_EQ(decoder.total_frame_count(), 1203400);
        REQUIRE_EQ(decoder.available_frame_count(), decoder.total_frame_count());
    }

    SUBCASE("korean file name without config")
    {
        Decoder decoder("../audio-samples/칠백KB.mp3");

        REQUIRE_EQ(decoder.format(), Format::F32);
        REQUIRE_EQ(decoder.channels(), 2);
        REQUIRE_EQ(decoder.sample_rate(), 32000);
        REQUIRE_EQ(decoder.total_frame_count(), 873216);
        REQUIRE_EQ(decoder.available_frame_count(), decoder.total_frame_count());
    }

    SUBCASE("korean file name with config")
    {
        Decoder decoder("../audio-samples/칠백KB.mp3", Format::S16, 1, 44100);

        REQUIRE_EQ(decoder.format(), Format::S16);
        REQUIRE_EQ(decoder.channels(), 1);
        REQUIRE_EQ(decoder.sample_rate(), 44100);
        REQUIRE_EQ(decoder.total_frame_count(), 1203400);
        REQUIRE_EQ(decoder.available_frame_count(), decoder.total_frame_count());
    }
}

TEST_CASE("[decoder] reads frames")
{
    const size_t FRAME_COUNT = 128;

    SUBCASE("without config")
    {
        Decoder decoder("../audio-samples/700KB.mp3");

        size_t buffer_size = get_format_size_in_bytes(decoder.format()) * decoder.channels() * FRAME_COUNT;
        std::vector<uint8_t> frames(buffer_size);
        size_t total_frames_read = 0;

        while (true)
        {
            size_t frames_read = decoder.read(frames.data(), FRAME_COUNT);
            total_frames_read += frames_read;

            if (frames_read == 0)
            {
                break;
            }
        }

        REQUIRE_EQ(total_frames_read, decoder.total_frame_count());
    }

    SUBCASE("with config")
    {
        Decoder decoder("../audio-samples/700KB.mp3", Format::S16, 1, 44100);

        size_t buffer_size = get_format_size_in_bytes(decoder.format()) * decoder.channels() * FRAME_COUNT;
        std::vector<uint8_t> frames(buffer_size);
        size_t total_frames_read = 0;

        while (true)
        {
            size_t frames_read = decoder.read(frames.data(), FRAME_COUNT);
            total_frames_read += frames_read;

            if (frames_read == 0)
            {
                break;
            }
        }

        REQUIRE(total_frames_read + FRAME_COUNT > decoder.total_frame_count());
    }
}
} // namespace tinyaudio
