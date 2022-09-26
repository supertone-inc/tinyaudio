#pragma once

#include "common.hpp"

#include <functional>

namespace tinyaudio
{
class Stream
{
public:
    using DataCallback = std::function<void(const void *input_frames, void *output_frames, size_t frame_count)>;
    using StopCallback = std::function<void()>;

    virtual ~Stream()
    {
    }

    virtual Format get_format() const = 0;
    virtual size_t get_channels() const = 0;
    virtual size_t get_sample_rate() const = 0;
    virtual size_t get_frame_count() const = 0;
    virtual bool is_started() const = 0;

    virtual void start(const DataCallback &data_callback, const StopCallback &stop_callback = nullptr) = 0;
    virtual void stop() = 0;
};
} // namespace tinyaudio
