#include "decoder.hpp"
#include "error.hpp"

#include <doctest.h>
#include <miniaudio.h>
#include <tinyaudio.hpp>

namespace tinyaudio
{
Tinyaudio::Tinyaudio()
{
}

Tinyaudio::~Tinyaudio()
{
}

Error::Error(const std::string &message)
    : std::runtime_error(message)
{
}

TEST_CASE("tinyaudio")
{
}
} // namespace tinyaudio
