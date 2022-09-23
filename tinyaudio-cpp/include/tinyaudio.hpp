#pragma once

#include <cstddef>
#include <functional>
#include <optional>
#include <string>

namespace tinyaudio
{
enum class Format
{
    UNKNOWN = 0,
    U8 = 1,
    S16 = 2,
    S24 = 3,
    S32 = 4,
    F32 = 5,
};

class Tinyaudio
{
public:
    using DataCallback = std::function<void(const void *input_frames, void *output_frames, size_t frame_count)>;

    Tinyaudio(
        bool offline,
        Format format,
        size_t channels,
        size_t sample_rate,
        size_t frame_count,
        std::optional<std::string> input_file_path,
        std::optional<std::string> output_file_path,
        bool looping_input_file
    );

    Tinyaudio(const Tinyaudio &) = delete;
    Tinyaudio &operator=(const Tinyaudio &) = delete;

    Tinyaudio(Tinyaudio &&);
    Tinyaudio &operator=(Tinyaudio &&);

    virtual ~Tinyaudio();

    bool is_offline() const;
    Format get_format() const;
    size_t get_channels() const;
    size_t get_sample_rate() const;
    size_t get_frame_count() const;
    bool is_looping_input_file() const;
    void set_looping_input_file(bool value);
    bool is_started() const;

    void start(const DataCallback &callback);
    void stop();

private:
    class Impl;
    Impl *impl;
};

struct AudioFileInfo {
    Format format;
    size_t channels;
    size_t sample_rate;
    size_t total_frame_count;
};

AudioFileInfo get_audio_file_info(const std::string &path);
} // namespace tinyaudio
