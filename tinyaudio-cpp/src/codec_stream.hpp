#pragma once

#include "common.hpp"
#include "decoder.hpp"
#include "encoder.hpp"
#include "stream.hpp"

#include <algorithm>
#include <atomic>
#include <exception>
#include <functional>
#include <string>
#include <vector>

namespace tinyaudio
{
class CodecStream : public Stream
{
public:
    CodecStream(
        const std::string &input_file_path,
        const std::string &output_file_path,
        EncodingFormat encoding_format,
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count
    )
        : decoder(input_file_path, format, channels, sample_rate, false)
        , encoder(output_file_path, encoding_format, format, channels, sample_rate)
        , frame_count(frame_count)
        , started(false)
    {
    }

    CodecStream(
        const std::wstring &input_file_path,
        const std::wstring &output_file_path,
        EncodingFormat encoding_format,
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count
    )
        : decoder(input_file_path, format, channels, sample_rate, false)
        , encoder(output_file_path, encoding_format, format, channels, sample_rate)
        , frame_count(frame_count)
        , started(false)
    {
    }

    EncodingFormat get_encoding_format() const
    {
        return encoder.get_encoding_format();
    }

    Format get_format() const
    {
        return decoder.get_format();
    }

    size_t get_channels() const
    {
        return decoder.get_channels();
    }

    size_t get_sample_rate() const
    {
        return decoder.get_sample_rate();
    }

    size_t get_frame_count() const
    {
        return frame_count;
    }

    bool is_started() const override
    {
        return started;
    }

    void start(const DataCallback &callback) override
    {
        try
        {
            started = true;

            auto bytes_per_frame = get_bytes_per_frame(get_format(), get_channels());
            std::vector<uint8_t> input_frames(bytes_per_frame * frame_count);
            std::vector<uint8_t> output_frames(bytes_per_frame * frame_count);

            while (started)
            {
                if (decoder.read(input_frames.data(), frame_count) == 0)
                {
                    break;
                }

                callback(input_frames.data(), output_frames.data(), frame_count);

                encoder.write(output_frames.data(), frame_count);
            }

            started = false;
        }
        catch (const std::exception &ex)
        {
            started = false;
            throw ex;
        }
    }

    void stop() override
    {
        started = false;
    }

private:
    Decoder decoder;
    Encoder encoder;
    size_t frame_count;
    std::atomic<bool> started;
};

namespace tests::codec_stream
{
const std::string INPUT_FILE_PATH = "../audio-samples/2MB.wav";
const std::string OUTPUT_FILE_PATH = "test-codec-stream.wav";
const EncodingFormat ENCODING_FORMAT = EncodingFormat::WAV;
const Format FORMAT = Format::F32;
const size_t CHANNELS = 1;
const size_t SAMPLE_RATE = 8000;
const size_t FRAME_COUNT = 128;

TEST_CASE("[codec_stream] works")
{
    CodecStream stream(INPUT_FILE_PATH, OUTPUT_FILE_PATH, ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT);

    REQUIRE_EQ(stream.get_encoding_format(), ENCODING_FORMAT);
    REQUIRE_EQ(stream.get_format(), FORMAT);
    REQUIRE_EQ(stream.get_channels(), CHANNELS);
    REQUIRE_EQ(stream.get_sample_rate(), SAMPLE_RATE);
    REQUIRE_EQ(stream.get_frame_count(), FRAME_COUNT);
    REQUIRE_EQ(stream.is_started(), false);

    stream.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            REQUIRE_EQ(stream.is_started(), true);
            std::copy_n(static_cast<const float *>(input_frames), frame_count, static_cast<float *>(output_frames));
        }
    );

    REQUIRE_EQ(stream.is_started(), false);
}
} // namespace tests::codec_stream
} // namespace tinyaudio
