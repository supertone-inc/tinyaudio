# tinyaudio-python

## Getting Started

### Installation

```sh
python -m pip install "git+https://github.com/supertone-inc/tinyaudio.git@<TINYAUDIO_VERSION>#egg=tinyaudio&subdirectory=tinyaudio-python"
```

### Example Usage

```python
import tinyaudio

audio = tinyaudio.Tinyaudio(
    True, # whether to process frames without device streaming
    tinyaudio.Format.F32, # sample format
    2, # channel count
    44100, # sample rate
    1024, # frame count
    "input.wav", # input file path
    "output.wav", # output file path
    False, # whether to loop input file
)

def data_callback(input_frames, output_frames):
    # bypassing input to output
    output_frames[:] = input_frames

audio.start(data_callback)
```

## Development

To build & run tests:

```sh
./test.sh
```
