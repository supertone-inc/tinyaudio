use crate::impl_from_ma_type;
use crate::ma_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use miniaudio_sys::*;
use std::mem::MaybeUninit;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    MiniaudioError(#[from] MiniaudioError),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Playback = ma_device_type_playback as _,
    Capture = ma_device_type_capture as _,
    Duplex = ma_device_type_duplex as _,
    Loopback = ma_device_type_loopback as _,
}

impl_from_ma_type!(DeviceType, ma_device_type);

#[repr(transparent)]
#[derive(Debug)]
pub struct DeviceConfig(ma_device_config);

impl DeviceConfig {
    pub fn new(
        device_type: DeviceType,
        format: Format,
        channels: usize,
        sample_rate: usize,
        frame_count: usize,
    ) -> DeviceConfig {
        let mut config = Self(unsafe { ma_device_config_init(device_type as _) });

        config.set_sample_rate(sample_rate);
        config.set_frame_count(frame_count);

        config.playback_mut().set_format(format);
        config.playback_mut().set_channels(channels);

        config.capture_mut().set_format(format);
        config.capture_mut().set_channels(channels);

        config
    }

    pub fn device_type(&self) -> DeviceType {
        self.0.deviceType.into()
    }

    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.0.deviceType = device_type as _;
    }

    pub fn sample_rate(&self) -> usize {
        self.0.sampleRate as _
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.0.sampleRate = sample_rate as _
    }

    pub fn frame_count(&self) -> usize {
        self.0.periodSizeInFrames as usize
    }

    pub fn set_frame_count(&mut self, frame_count: usize) {
        self.0.periodSizeInFrames = frame_count as _;
    }

    #[inline]
    pub fn playback(&self) -> &DeviceConfigPlayback {
        unsafe {
            &*(&self.0.playback as *const MaDeviceConfigPlayback as *const DeviceConfigPlayback)
        }
    }

    #[inline]
    pub fn playback_mut(&mut self) -> &mut DeviceConfigPlayback {
        unsafe {
            &mut *(&mut self.0.playback as *mut MaDeviceConfigPlayback as *mut DeviceConfigPlayback)
        }
    }

    #[inline]
    pub fn capture(&self) -> &DeviceConfigCapture {
        unsafe { &*(&self.0.capture as *const MaDeviceConfigCapture as *const DeviceConfigCapture) }
    }

    #[inline]
    pub fn capture_mut(&mut self) -> &mut DeviceConfigCapture {
        unsafe {
            &mut *(&mut self.0.capture as *mut MaDeviceConfigCapture as *mut DeviceConfigCapture)
        }
    }
}

type MaDeviceConfigPlayback = ma_device_config__bindgen_ty_1;

#[repr(transparent)]
#[derive(Debug)]
pub struct DeviceConfigPlayback(MaDeviceConfigPlayback);

impl DeviceConfigPlayback {
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
}

type MaDeviceConfigCapture = ma_device_config__bindgen_ty_2;

#[repr(transparent)]
#[derive(Debug)]
pub struct DeviceConfigCapture(MaDeviceConfigCapture);

impl DeviceConfigCapture {
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
}

#[derive(Debug)]
pub struct Device(Box<ma_device>);

impl Device {
    pub fn new(config: &DeviceConfig) -> Result<Self, Error> {
        let mut device = Box::new(MaybeUninit::<ma_device>::uninit());

        ma_result!(ma_device_init(
            std::ptr::null_mut(),
            &config.0,
            device.as_mut_ptr(),
        ))?;

        Ok(unsafe { std::mem::transmute(device) })
    }

    pub fn device_type(&self) -> DeviceType {
        self.0.type_.into()
    }

    pub fn format(&self) -> Format {
        match self.device_type() {
            DeviceType::Playback => self.0.playback.format.into(),
            _ => self.0.capture.format.into(),
        }
    }

    pub fn channels(&self) -> usize {
        match self.device_type() {
            DeviceType::Playback => self.0.playback.channels as _,
            _ => self.0.capture.channels as _,
        }
    }

    pub fn sample_rate(&self) -> usize {
        self.0.sampleRate as _
    }

    pub fn frame_count(&self) -> usize {
        match self.device_type() {
            DeviceType::Playback => self.0.playback.internalPeriodSizeInFrames as _,
            _ => self.0.capture.internalPeriodSizeInFrames as _,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        Ok(ma_result!(ma_device_start(self.0.as_mut()))?)
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        Ok(ma_result!(ma_device_start(self.0.as_mut()))?)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { ma_device_uninit(self.0.as_mut()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMAT: Format = Format::F32;
    const CHANNELS: usize = 1;
    const SAMPLE_RATE: usize = 44100;
    const FRAME_COUNT: usize = 128;

    #[test]
    fn test_metadata() {
        let test = |device_type| {
            let device = Device::new(&DeviceConfig::new(
                device_type,
                FORMAT,
                CHANNELS,
                SAMPLE_RATE,
                FRAME_COUNT,
            ))
            .unwrap();

            assert_eq!(device.device_type(), device_type);
            assert_eq!(device.format(), FORMAT);
            assert_eq!(device.channels(), CHANNELS);
            assert_eq!(device.sample_rate(), SAMPLE_RATE);
            assert_eq!(device.frame_count(), FRAME_COUNT);
        };

        test(DeviceType::Playback);
        test(DeviceType::Capture);
        test(DeviceType::Duplex);
    }
}
