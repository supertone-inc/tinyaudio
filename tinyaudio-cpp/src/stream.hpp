#pragma once

#include "common.hpp"

#include <functional>

namespace tinyaudio
{
class Stream
{
public:
    using DataCallback = std::function<void(const void *input_frames, void *output_frames, size_t frame_count)>;

    virtual bool is_started() const = 0;
    virtual void start(const DataCallback &callback) = 0;
    virtual void stop() = 0;
};
} // namespace tinyaudio
