use crate::miniaudio_error::to_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use miniaudio_sys::*;
use std::ffi::CString;
use std::path::Path;
use thiserror::Error;

#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStringExt;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error(transparent)]
    FfiNulError(#[from] std::ffi::NulError),

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
    format: Format,
    channels: usize,
    sample_rate: usize,
    total_frame_count: usize,
}

impl Decoder {
    pub fn new<P: AsRef<Path>>(
        file_path: P,
        config: Option<DecoderConfig>,
    ) -> Result<Self, DecoderError> {
        let file_path = CString::new(file_path.as_ref().as_os_str().as_bytes())?;

        let config = match config {
            Some(config) => &config.0,
            None => std::ptr::null(),
        };

        let mut decoder = ma_decoder::default();

        unsafe {
            to_result(ma_decoder_init_file(
                file_path.as_ptr(),
                config,
                &mut decoder,
            ))?;
        }

        let mut format = 0;
        let mut channels = 0;
        let mut sample_rate = 0;
        let mut total_frame_count = 0;

        unsafe {
            to_result(ma_decoder_get_data_format(
                &mut decoder,
                &mut format,
                &mut channels,
                &mut sample_rate,
                std::ptr::null_mut(),
                0,
            ))?;

            to_result(ma_decoder_get_length_in_pcm_frames(
                &mut decoder,
                &mut total_frame_count,
            ))?;
        }

        Ok(Self {
            raw: decoder,
            format: format.into(),
            channels: channels as _,
            sample_rate: sample_rate as _,
            total_frame_count: total_frame_count as _,
        })
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    pub fn total_frame_count(&self) -> usize {
        self.total_frame_count
    }

    pub fn available_frame_count(&mut self) -> Result<usize, DecoderError> {
        let mut available_frames = 0;

        unsafe {
            to_result(ma_decoder_get_available_frames(
                &mut self.raw,
                &mut available_frames,
            ))?;
        }

        Ok(available_frames as _)
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
            to_result(ma_decoder_read_pcm_frames(
                &mut self.raw,
                frames.as_mut_ptr() as _,
                (frames.len() / self.channels) as _,
                &mut frames_read,
            ))?;
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

    #[test]
    fn it_works() {
        let file_path = "../audio-samples/file_example_WAV_1MG.wav";
        let config = DecoderConfig::new(Format::F32, 2, 44100);
        let mut decoder = Decoder::new(file_path, Some(config)).unwrap();

        assert_eq!(decoder.format(), config.format());
        assert_eq!(decoder.channels(), config.channels());
        assert_eq!(decoder.sample_rate(), config.sample_rate());
        assert!(decoder.total_frame_count() > 0);
        assert_eq!(
            decoder.available_frame_count().unwrap(),
            decoder.total_frame_count()
        );

        let mut frames = vec![0_f32; 1024];
        let mut frames_read = 0;

        while decoder.available_frame_count().unwrap() > 0 {
            frames_read += decoder.read(&mut frames).unwrap();
        }

        assert_eq!(frames_read, decoder.total_frame_count());

        decoder.seek(0).unwrap();

        assert_eq!(
            decoder.available_frame_count().unwrap(),
            decoder.total_frame_count()
        );
    }
}
