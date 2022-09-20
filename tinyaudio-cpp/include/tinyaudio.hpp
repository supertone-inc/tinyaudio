#pragma once

#include <stdexcept>

namespace tinyaudio
{
class Tinyaudio
{
public:
    Tinyaudio();
    virtual ~Tinyaudio();
};

class Error : public std::runtime_error
{
public:
    Error(const std::string &message);
};

enum class Format
{
    UNKNOWN,
    U8,
    S16,
    S24,
    S32,
    F32,
};
} // namespace tinyaudio
