#include "codec_stream.hpp"
#include "common.hpp"
#include "decoder.hpp"
#include "encoder.hpp"

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
