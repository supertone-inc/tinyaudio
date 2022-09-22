#pragma once

#include <miniaudio.h>
#include <string>
#include <tinyaudio.hpp>

namespace tinyaudio
{
class Error : public std::runtime_error
{
public:
    Error(const std::string &message)
        : std::runtime_error(message)
    {
    }
};

inline void check_result(ma_result ma_result)
{
    if (ma_result == MA_SUCCESS)
    {
        return;
    }

    throw Error(ma_result_description(ma_result));
}

size_t get_bytes_per_sample(Format format)
{
    return ma_get_bytes_per_sample(static_cast<ma_format>(format));
}

size_t get_bytes_per_frame(Format format, size_t channels)
{
    return ma_get_bytes_per_frame(static_cast<ma_format>(format), channels);
}
} // namespace tinyaudio

#include <doctest.h>
