use crate::Decoder;
use crate::DecoderConfig;
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
    DecoderError(#[from] crate::decoder::Error),

    #[error(transparent)]
    EncoderError(#[from] crate::encoder::Error),
}

pub struct CodecStream {
    decoder: Decoder,
    encoder: Encoder,
    frame_count: usize,
}

impl CodecStream {
    pub fn new<P: AsRef<Path>>(
        input_file_path: P,
        output_file_path: P,
        format: Format,
        channels: usize,
        sample_rate: usize,
        frame_count: usize,
    ) -> Result<Self, Error> {
        let decoder = Decoder::new(
            input_file_path,
            Some(&DecoderConfig::new(format, channels, sample_rate)),
        )?;

        let encoder = Encoder::new(
            output_file_path,
            &EncoderConfig::new(
                EncodingFormat::Wav,
                decoder.format(),
                decoder.channels(),
                decoder.sample_rate(),
            ),
        )?;

        Ok(Self {
            decoder,
            encoder,
            frame_count,
        })
    }

    pub fn format(&self) -> Format {
        self.decoder.format()
    }

    pub fn channels(&self) -> usize {
        self.decoder.channels()
    }

    pub fn sample_rate(&self) -> usize {
        self.decoder.sample_rate()
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }
}

impl Stream for CodecStream {
    type Error = Error;

    fn start<StreamCallback>(&mut self, callback: StreamCallback) -> Result<(), Self::Error>
    where
        StreamCallback: Fn(&Frames, &mut FramesMut),
    {
        let buffer_size = self.format().size_in_bytes() * self.channels() * self.frame_count();
        let mut input_buffer = vec![0_u8; buffer_size];
        let mut output_buffer = vec![0_u8; buffer_size];

        loop {
            match self.decoder.read(&mut FramesMut::wrap(
                &mut input_buffer,
                self.format(),
                self.channels(),
            ))? {
                0 => break,
                _ => {
                    callback(
                        &Frames::wrap(&input_buffer, self.format(), self.channels()),
                        &mut FramesMut::wrap(&mut output_buffer, self.format(), self.channels()),
                    );

                    self.encoder.write(&Frames::wrap(
                        &output_buffer,
                        self.format(),
                        self.channels(),
                    ))?;
                }
            }
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../audio-samples/2MB.wav");
    const OUTPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test-2MB.wav");
    const FORMAT: Format = Format::F32;
    const CHANNELS: usize = 1;
    const SAMPLE_RATE: usize = 8000;
    const FRAME_COUNT: usize = 128;

    #[test]
    fn it_works() {
        let mut stream = CodecStream::new(
            INPUT_FILE_PATH,
            OUTPUT_FILE_PATH,
            FORMAT,
            CHANNELS,
            SAMPLE_RATE,
            FRAME_COUNT,
        )
        .unwrap();

        stream
            .start(|input_frames, output_frames| {
                output_frames
                    .as_bytes_mut()
                    .copy_from_slice(input_frames.as_bytes())
            })
            .unwrap();
    }
}
