use crate::device::DeviceConfig;
use crate::device::DeviceType;
use crate::Decoder;
use crate::DecoderConfig;
use crate::Device;
use crate::Encoder;
use crate::EncoderConfig;
use crate::EncodingFormat;
use crate::Format;
use crate::Frames;
use crate::FramesMut;
use crate::Stream;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DeviceError(#[from] crate::device::Error),

    #[error(transparent)]
    DecoderError(#[from] crate::decoder::Error),

    #[error(transparent)]
    EncoderError(#[from] crate::encoder::Error),
}

struct Codec {
    decoder: Option<(Decoder, Vec<u8>)>,
    encoder: Option<Encoder>,
}

pub struct DeviceStream {
    device: Device,
}

impl DeviceStream {
    pub fn new<P: AsRef<Path>>(
        format: Format,
        channels: usize,
        sample_rate: usize,
        frame_count: usize,
        input_file_path: Option<P>,
        output_file_path: Option<P>,
        loop_input_file: bool,
    ) -> Result<Self, Error> {
        let decoder = match input_file_path {
            Some(ref input_file_path) => {
                let mut decoder = Decoder::new(
                    input_file_path,
                    Some(DecoderConfig::new(format, channels, sample_rate)),
                )?;

                decoder.set_looping(loop_input_file);

                let buffer = vec![0_u8; format.size_in_bytes() * channels * frame_count];

                Some((decoder, buffer))
            }
            None => None,
        };

        let encoder = match output_file_path {
            Some(output_file_path) => Some(Encoder::new(
                output_file_path,
                EncoderConfig::new(EncodingFormat::Wav, format, channels, sample_rate),
            )?),
            None => None,
        };

        let device = Device::new(DeviceConfig::new(
            match input_file_path {
                Some(_) => DeviceType::Playback,
                None => DeviceType::Duplex,
            },
            format,
            channels,
            sample_rate,
            frame_count,
            Some(Box::new(Codec { decoder, encoder })),
        ))?;

        Ok(Self { device })
    }

    pub fn format(&self) -> Format {
        self.device.format()
    }

    pub fn channels(&self) -> usize {
        self.device.channels()
    }

    pub fn sample_rate(&self) -> usize {
        self.device.sample_rate()
    }

    pub fn frame_count(&self) -> usize {
        self.device.frame_count()
    }
}

impl Stream for DeviceStream {
    type Error = Error;

    fn start<StreamCallback>(&mut self, callback: StreamCallback) -> Result<(), Self::Error>
    where
        StreamCallback: Fn(&Frames, &mut FramesMut) + 'static,
    {
        Ok(self
            .device
            .start(move |_, user_data, input_frames, output_frames| {
                let codec = user_data.unwrap().downcast_mut::<Codec>().unwrap();

                let input_buffer = if let Some((ref mut decoder, ref mut buffer)) = codec.decoder {
                    decoder
                        .read(&mut FramesMut::wrap(
                            buffer,
                            input_frames.format(),
                            input_frames.channels(),
                        ))
                        .unwrap();

                    buffer
                } else {
                    input_frames.as_bytes()
                };

                callback(
                    &Frames::wrap(input_buffer, input_frames.format(), input_frames.channels()),
                    output_frames,
                );

                if let Some(ref mut encoder) = codec.encoder {
                    encoder
                        .write(&Frames::wrap(
                            output_frames.as_bytes(),
                            output_frames.format(),
                            output_frames.channels(),
                        ))
                        .unwrap();
                }
            })?)
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        Ok(self.device.stop()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../audio-samples/2MB.wav");
    const OUTPUT_FILE_PATH: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/test-device-stream-2MB.wav");
    const FORMAT: Format = Format::F32;
    const CHANNELS: usize = 1;
    const SAMPLE_RATE: usize = 8000;
    const FRAME_COUNT: usize = 128;

    #[test]
    fn it_works() {
        let mut stream = DeviceStream::new(
            FORMAT,
            CHANNELS,
            SAMPLE_RATE,
            FRAME_COUNT,
            Some(INPUT_FILE_PATH),
            Some(OUTPUT_FILE_PATH),
            false,
        )
        .unwrap();

        stream
            .start(|input_frames, output_frames| {
                output_frames
                    .as_bytes_mut()
                    .copy_from_slice(input_frames.as_bytes())
            })
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
