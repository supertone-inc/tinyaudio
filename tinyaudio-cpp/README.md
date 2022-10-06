# tinyaudio-cpp

## Getting Started

### Integration with CMake

```cmake
include(FetchContent)

FetchContent_Declare(
  tinyaudio
  GIT_REPOSITORY https://github.com/supertone-inc/tinyaudio.git
  GIT_TAG <TINYAUDIO_VERSION>
)

FetchContent_GetProperties(tinyaudio)

if(NOT tinyaudio_POPULATED)
  FetchContent_Populate(tinyaudio)
  add_subdirectory(${tinyaudio_SOURCE_DIR}/tinyaudio-cpp ${tinyaudio_BINARY_DIR})
endif()

target_link_libraries(<YOUR_TARGET> tinyaudio)

if(UNIX AND NOT APPLE)
  target_link_libraries(<YOUR_TARGET> dl pthread m)
endif()
```

### Example Usage

```c++
#include <tinyaudio.hpp>

tinyaudio::Tinyaudio audio(
    true, // whether to process frames without device streaming
    tinyaudio::Format::F32, // sample format
    2, // channel count
    44100, // sample rate
    1024, // frame count
    "input.wav", // input file path
    "output.wav", // output file path
    false // whether to loop input file
);

audio.start(
    [&](const void *input_frames, void *output_frames, size_t frame_count)
    {
        // bypassing input to output
        std::copy_n(
            reinterpret_cast<const float *>(input_frames),
            audio.get_channels() * frame_count,
            reinterpret_cast<float *>(output_frames)
        );
    }
);
```

## Development

To build & run tests:

```sh
./test.sh
```
