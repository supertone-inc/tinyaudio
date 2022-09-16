use crate::miniaudio_error::to_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use miniaudio_sys::*;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncoderError {
    #[error("{0:#?}")]
    MiniaudioError(MiniaudioError),
}

impl From<MiniaudioError> for EncoderError {
    fn from(err: MiniaudioError) -> Self {
        EncoderError::MiniaudioError(err)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingFormat {
    Unknown = ma_encoding_format_unknown,
    Wav = ma_encoding_format_wav,
    Flac = ma_encoding_format_flac,
    Mp3 = ma_encoding_format_mp3,
    Vorbis = ma_encoding_format_vorbis,
}

impl From<ma_encoding_format> for EncodingFormat {
    fn from(encoding_format: ma_encoding_format) -> Self {
        unsafe { std::mem::transmute(encoding_format) }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EncoderConfig(ma_encoder_config);

impl EncoderConfig {
    pub fn new(
        encoding_format: EncodingFormat,
        format: Format,
        channels: usize,
        sample_rate: usize,
    ) -> Self {
        Self(unsafe {
            ma_encoder_config_init(
                encoding_format as _,
                format as _,
                channels as _,
                sample_rate as _,
            )
        })
    }

    pub fn encoding_format(&self) -> EncodingFormat {
        self.0.encodingFormat.into()
    }

    pub fn set_encoding_format(&mut self, encoding_format: EncodingFormat) {
        self.0.encodingFormat = encoding_format as _;
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

#[derive(Debug)]
pub struct Encoder(Box<ma_encoder>);

impl Encoder {
    pub fn new<P: AsRef<Path>>(file_path: P, config: &EncoderConfig) -> Result<Self, EncoderError> {
        Ok(Self(unsafe {
            let file_path =
                CString::from_vec_unchecked(file_path.as_ref().to_string_lossy().as_bytes().into());

            let mut encoder = Box::new(MaybeUninit::<ma_encoder>::uninit());

            to_result(ma_encoder_init_file(
                file_path.as_ptr(),
                &config.0,
                encoder.as_mut_ptr(),
            ))?;

            std::mem::transmute(encoder)
        }))
    }

    pub fn encoding_format(&self) -> EncodingFormat {
        self.0.config.encodingFormat.into()
    }

    pub fn format(&self) -> Format {
        self.0.config.format.into()
    }

    pub fn channels(&self) -> usize {
        self.0.config.channels as _
    }

    pub fn sample_rate(&self) -> usize {
        self.0.config.sampleRate as _
    }

    pub fn write<T>(&mut self, frames: &[T]) -> Result<usize, EncoderError> {
        let mut frames_written = 0;

        unsafe {
            to_result(ma_encoder_write_pcm_frames(
                self.0.as_mut(),
                frames.as_ptr() as _,
                (frames.len() / self.channels()) as _,
                &mut frames_written,
            ))?
        };

        Ok(frames_written as _)
    }

    pub fn close(&mut self) {
        unsafe {
            if !self.0.data.vfs.file.is_null() {
                ma_encoder_uninit(self.0.as_mut());
            }
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUTPUT_AUDIO_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test.wav");
    const ENCODING_FORMAT: EncodingFormat = EncodingFormat::Wav;
    const FORMAT: Format = Format::F32;
    const CHANNELS: usize = 2;
    const SAMPLE_RATE: usize = 44100;
    const FRAME_COUNT: usize = 128;
    const DURATION_IN_SECS: usize = 1;
    const LOOP_COUNT: usize = DURATION_IN_SECS * SAMPLE_RATE / FRAME_COUNT;

    #[test]
    fn test_metadata() {
        let config = EncoderConfig::new(ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);
        let encoder = Encoder::new(OUTPUT_AUDIO_FILE_PATH, &config).unwrap();

        assert_eq!(encoder.encoding_format(), ENCODING_FORMAT);
        assert_eq!(encoder.format(), FORMAT);
        assert_eq!(encoder.channels(), CHANNELS);
        assert_eq!(encoder.sample_rate(), SAMPLE_RATE);
    }

    #[test]
    fn test_write() {
        let config = EncoderConfig::new(ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);
        let mut encoder = Encoder::new(OUTPUT_AUDIO_FILE_PATH, &config).unwrap();

        let mut waveform = unsafe {
            let config = ma_waveform_config_init(
                FORMAT as _,
                CHANNELS as _,
                SAMPLE_RATE as _,
                ma_waveform_type_sine,
                0.5,
                440.0,
            );

            let mut waveform = std::mem::MaybeUninit::<ma_waveform>::uninit();

            to_result(ma_waveform_init(&config, waveform.as_mut_ptr())).unwrap();

            waveform.assume_init()
        };

        let mut frames = vec![0_f32; CHANNELS * FRAME_COUNT];
        let mut total_frames_written = 0;

        for _ in 0..LOOP_COUNT {
            unsafe {
                ma_waveform_read_pcm_frames(
                    &mut waveform,
                    frames.as_mut_ptr() as _,
                    (frames.len() / CHANNELS) as _,
                    std::ptr::null_mut(),
                )
            };

            total_frames_written += encoder.write(&frames).unwrap();
        }

        assert_eq!(total_frames_written, LOOP_COUNT * FRAME_COUNT);

        unsafe { ma_waveform_uninit(&mut waveform) };
    }

    #[test]
    fn test_close() {
        let config = EncoderConfig::new(ENCODING_FORMAT, FORMAT, CHANNELS, SAMPLE_RATE);
        let mut encoder = Encoder::new(OUTPUT_AUDIO_FILE_PATH, &config).unwrap();

        unsafe {
            assert!(!encoder.0.data.vfs.file.is_null());

            encoder.close();

            assert!(encoder.0.data.vfs.file.is_null());
        }
    }
}
