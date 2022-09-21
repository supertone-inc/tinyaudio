#include "codec_stream.hpp"
#include "decoder.hpp"
#include "encoder.hpp"
#include "error.hpp"
#include "format.hpp"

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

namespace tests
{
TEST_CASE("tinyaudio")
{
}
} // namespace tests
} // namespace tinyaudio
