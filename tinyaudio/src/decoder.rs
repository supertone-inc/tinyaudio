use crate::miniaudio_error::to_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use miniaudio_sys::*;
use std::ffi::CString;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("{0:#?}")]
    MiniaudioError(MiniaudioError),
}

impl From<MiniaudioError> for DecoderError {
    fn from(err: MiniaudioError) -> Self {
        DecoderError::MiniaudioError(err)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct DecoderConfig(ma_decoder_config);

impl DecoderConfig {
    pub fn new(output_format: Format, output_channels: usize, output_sample_rate: usize) -> Self {
        Self(unsafe {
            ma_decoder_config_init(
                output_format as _,
                output_channels as _,
                output_sample_rate as _,
            )
        })
    }

    pub fn format(&self) -> Format {
        self.0.format.into()
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    pub fn channels(&self) -> usize {
        self.0.channels as _
    }

    pub fn set_channels(&mut self, channels: usize) {
        self.0.channels = channels as _;
    }

    pub fn sample_rate(&self) -> usize {
        self.0.sampleRate as _
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.0.sampleRate = sample_rate as _;
    }
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self(unsafe { ma_decoder_config_init_default() })
    }
}

pub struct Decoder {
    raw: ma_decoder,
    total_frame_count: usize,
}

impl Decoder {
    pub fn new<P: AsRef<Path>>(
        file_path: P,
        config: Option<DecoderConfig>,
    ) -> Result<Self, DecoderError> {
        let file_path = unsafe {
            CString::from_vec_unchecked(file_path.as_ref().to_string_lossy().as_bytes().into())
        };

        let config = match config {
            Some(config) => &config.0,
            None => std::ptr::null(),
        };

        let mut decoder = unsafe {
            let mut decoder = std::mem::MaybeUninit::<ma_decoder>::uninit();

            to_result(ma_decoder_init_file(
                file_path.as_ptr(),
                config,
                decoder.as_mut_ptr(),
            ))?;

            decoder.assume_init()
        };

        let mut total_frame_count = 0;
        unsafe {
            to_result(ma_decoder_get_length_in_pcm_frames(
                &mut decoder,
                &mut total_frame_count,
            ))?;
        }

        Ok(Self {
            raw: decoder,
            total_frame_count: total_frame_count as _,
        })
    }

    pub fn format(&self) -> Format {
        self.raw.outputFormat.into()
    }

    pub fn channels(&self) -> usize {
        self.raw.outputChannels as _
    }

    pub fn sample_rate(&self) -> usize {
        self.raw.outputSampleRate as _
    }

    pub fn total_frame_count(&self) -> usize {
        self.total_frame_count
    }

    pub fn available_frame_count(&mut self) -> usize {
        let read_pointer_in_pcm_frames = self.raw.readPointerInPCMFrames as usize;

        if self.total_frame_count < read_pointer_in_pcm_frames {
            return 0;
        }

        self.total_frame_count - read_pointer_in_pcm_frames
    }

    pub fn seek(&mut self, frame_index: usize) -> Result<(), DecoderError> {
        unsafe {
            Ok(to_result(ma_decoder_seek_to_pcm_frame(
                &mut self.raw,
                frame_index as _,
            ))?)
        }
    }

    pub fn read<T>(&mut self, frames: &mut [T]) -> Result<usize, DecoderError> {
        let mut frames_read = 0;

        unsafe {
            match to_result(ma_decoder_read_pcm_frames(
                &mut self.raw,
                frames.as_mut_ptr() as _,
                (frames.len() / self.channels()) as _,
                &mut frames_read,
            )) {
                Ok(_) | Err(MiniaudioError::AtEnd) => {}
                err => err?,
            }
        }

        Ok(frames_read as _)
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            ma_decoder_uninit(&mut self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_AUDIO_FILE_PATH: &str = "../audio-samples/2MB.wav";
    const FRAME_COUNT: usize = 128;

    #[test]
    fn test_metadata() {
        let mut decoder = Decoder::new(SAMPLE_AUDIO_FILE_PATH, None).unwrap();

        assert_ne!(decoder.format(), Format::Unknown);
        assert!(decoder.channels() > 0);
        assert!(decoder.sample_rate() > 0);
        assert!(decoder.total_frame_count() > 0);
        assert_eq!(decoder.available_frame_count(), decoder.total_frame_count());
    }

    #[test]
    fn test_seek() {
        let mut decoder = Decoder::new(SAMPLE_AUDIO_FILE_PATH, None).unwrap();

        decoder.seek(decoder.total_frame_count()).unwrap();
        assert_eq!(decoder.available_frame_count(), 0);

        decoder.seek(0).unwrap();
        assert_eq!(decoder.available_frame_count(), decoder.total_frame_count());
    }

    #[test]
    fn test_read() {
        let mut decoder = Decoder::new(SAMPLE_AUDIO_FILE_PATH, None).unwrap();

        let mut frames = vec![0_f32; FRAME_COUNT];
        let mut total_frames_read = 0;

        loop {
            match decoder.read(&mut frames).unwrap() {
                0 => break,
                frames_read => total_frames_read += frames_read,
            }
        }

        assert_eq!(total_frames_read, decoder.total_frame_count());
    }

    #[test]
    fn test_read_with_config() {
        let config = DecoderConfig::new(Format::F32, 1, 8000);
        let mut decoder = Decoder::new(SAMPLE_AUDIO_FILE_PATH, Some(config)).unwrap();

        assert_eq!(decoder.format(), config.format());
        assert_eq!(decoder.channels(), config.channels());
        assert_eq!(decoder.sample_rate(), config.sample_rate());
        assert!(decoder.total_frame_count() > 0);
        assert_eq!(decoder.available_frame_count(), decoder.total_frame_count());

        let mut frames = vec![0_f32; FRAME_COUNT];
        let mut total_frames_read = 0;

        loop {
            match decoder.read(&mut frames).unwrap() {
                0 => break,
                frames_read => total_frames_read += frames_read,
            }
        }

        assert!(total_frames_read + frames.len() >= decoder.total_frame_count());
    }
}
