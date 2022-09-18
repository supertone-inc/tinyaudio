use crate::impl_from_ma_type;
use crate::ma_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use miniaudio_sys::*;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    MiniaudioError(#[from] MiniaudioError),
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Playback = ma_device_type_playback,
    Capture = ma_device_type_capture,
    Duplex = ma_device_type_duplex,
    Loopback = ma_device_type_loopback,
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

#[repr(transparent)]
#[derive(Debug)]
pub struct RawDevice(ma_device);

impl RawDevice {
    pub fn new(config: &DeviceConfig) -> Result<Arc<Self>, Error> {
        let device = Arc::new(MaybeUninit::<ma_device>::uninit());

        ma_result!(ma_device_init(
            std::ptr::null_mut(),
            &config.0,
            Arc::deref(&device).as_ptr() as *mut _,
        ))?;

        Ok(unsafe { std::mem::transmute(device) })
    }

    pub fn start(&self) -> Result<(), Error> {
        Ok(ma_result!(ma_device_start(&self.0 as *const _ as _))?)
    }

    pub fn stop(&self) -> Result<(), Error> {
        Ok(ma_result!(ma_device_stop(&self.0 as *const _ as _))?)
    }
}

impl Drop for RawDevice {
    fn drop(&mut self) {
        unsafe { ma_device_uninit(&mut self.0) };
    }
}

#[derive(Debug)]
pub struct Device(Arc<RawDevice>);

impl Device {
    pub fn new(config: &DeviceConfig) -> Result<Self, Error> {
        Ok(Self(RawDevice::new(config)?))
    }

    pub fn device_type(&self) -> DeviceType {
        self.0 .0.type_.into()
    }

    pub fn sample_rate(&self) -> usize {
        self.0 .0.sampleRate as _
    }

    pub fn start(&self) -> Result<(), Error> {
        self.0.start()
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.0.stop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    const FORMAT: Format = Format::F32;
    const CHANNELS: usize = 1;
    const SAMPLE_RATE: usize = 44100;
    const FRAME_COUNT: usize = 128;

    #[test]
    #[serial]
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
            assert_eq!(device.sample_rate(), SAMPLE_RATE);
        };

        test(DeviceType::Duplex);
    }
}
