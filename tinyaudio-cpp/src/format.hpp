#pragma once

#include <miniaudio.h>
#include <tinyaudio.hpp>

namespace tinyaudio
{
size_t get_bytes_per_sample(Format format)
{
    return ma_get_bytes_per_sample(static_cast<ma_format>(format));
}

size_t get_bytes_per_frame(Format format, size_t channels)
{
    return ma_get_bytes_per_frame(static_cast<ma_format>(format), channels);
}
} // namespace tinyaudio
