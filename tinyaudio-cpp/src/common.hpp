#pragma once

// clang-format off
#define MINIAUDIO_IMPLEMENTATION
#include <miniaudio.h>
// clang-format on

#include <stdexcept>
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

class MiniaudioError : public Error
{
public:
    MiniaudioError(ma_result result)
        : result(result)
        , Error(ma_result_description(result))
    {
    }

    ma_result get_miniaudio_result() const
    {
        return result;
    }

private:
    ma_result result;
};

inline void check_result(ma_result result)
{
    if (result == MA_SUCCESS)
    {
        return;
    }

    throw MiniaudioError(result);
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

#ifdef TINYAUDIO_BUILD_TESTS
#include <chrono>
#include <condition_variable>
#include <doctest.h>
#include <mutex>
#include <thread>

namespace tinyaudio::tests
{
using namespace std::chrono_literals;

const auto WAIT_TIMEOUT = 0.1s;

std::mutex mutex;
std::condition_variable cv;

template <typename Duration = std::chrono::duration<double>>
void wait(const Duration &timeout = WAIT_TIMEOUT)
{
    std::unique_lock<std::mutex> lock(mutex);
    cv.wait_for(lock, timeout);
}

void notify()
{
    std::unique_lock<std::mutex> lock(mutex);
    cv.notify_all();
}
} // namespace tinyaudio::tests
#endif