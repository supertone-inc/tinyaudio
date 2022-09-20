#pragma once

#include <tinyaudio.hpp>

namespace tinyaudio
{
size_t get_format_size_in_bytes(Format format)
{
    switch (format)
    {
    case Format::U8:
        return 1;
    case Format::S16:
        return 2;
    case Format::S24:
        return 3;
    case Format::S32:
        return 4;
    case Format::F32:
        return 4;
    default:
        return 0;
    }
}
} // namespace tinyaudio
