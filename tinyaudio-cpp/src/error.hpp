#pragma once

#include <miniaudio.h>
#include <string>
#include <tinyaudio.hpp>

namespace tinyaudio
{
class MiniaudioError : Error
{
public:
    MiniaudioError(ma_result ma_result)
        : Error(ma_result_description(ma_result))
    {
    }
};

inline void check_result(ma_result ma_result)
{
    if (ma_result == MA_SUCCESS)
    {
        return;
    }

    throw MiniaudioError(ma_result);
}
} // namespace tinyaudio
