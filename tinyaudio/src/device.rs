use crate::impl_from_ma_type;
use crate::ma_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use crate::Frames;
use crate::FramesMut;
use miniaudio_sys::*;
use std::any::Any;
use std::ffi::c_void;
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

pub type DataCallback = dyn Fn(&Device, Option<&mut UserData>, &Frames, &mut FramesMut);
pub type UserData = dyn Any;

struct DeviceUserData<'a> {
    device: Option<&'a Device>,
    data_callback: Option<Box<DataCallback>>,
    user_data: Option<Box<UserData>>,
}

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
        user_data: Option<Box<UserData>>,
    ) -> DeviceConfig {
        let mut config = unsafe { ma_device_config_init(device_type as _) };

        config.sampleRate = sample_rate as _;
        config.periodSizeInFrames = frame_count as _;
        config.dataCallback = Some(device_data_callback);
        config.pUserData = Box::into_raw(Box::new(DeviceUserData {
            device: None,
            data_callback: None,
            user_data,
        })) as _;

        config.playback.format = format as _;
        config.playback.channels = channels as _;

        config.capture.format = format as _;
        config.capture.channels = channels as _;

        unsafe { std::mem::transmute(config) }
    }

    pub fn device_type(&self) -> DeviceType {
        self.0.deviceType.into()
    }

    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.0.deviceType = device_type as _;
    }

    pub fn format(&self) -> Format {
        match self.device_type() {
            DeviceType::Playback => self.0.playback.format.into(),
            _ => self.0.capture.format.into(),
        }
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.playback.format = format as _;
        self.0.capture.format = format as _;
    }

    pub fn channels(&self) -> usize {
        match self.device_type() {
            DeviceType::Playback => self.0.playback.channels as _,
            _ => self.0.capture.channels as _,
        }
    }

    pub fn set_channels(&mut self, channels: usize) {
        self.0.playback.channels = channels as _;
        self.0.capture.channels = channels as _;
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

    pub fn user_data(&self) -> Option<&UserData> {
        let device_user_data = unsafe { &*self.0.pUserData.cast::<DeviceUserData>() };
        device_user_data.user_data.as_deref()
    }

    pub fn set_user_data(&mut self, user_data: Option<Box<UserData>>) {
        let device_user_data = unsafe { &mut *self.0.pUserData.cast::<DeviceUserData>() };
        device_user_data.user_data = user_data;
    }
}

impl Drop for DeviceConfig {
    fn drop(&mut self) {
        if !self.0.pUserData.is_null() {
            drop(unsafe { Box::from_raw(self.0.pUserData.cast::<DeviceUserData>()) })
        }
    }
}

unsafe extern "C" fn device_data_callback(
    device_ptr: *mut ma_device,
    output_ptr: *mut c_void,
    input_ptr: *const c_void,
    frame_count: u32,
) {
    let ma_device = &mut *device_ptr;

    let input_format = ma_device.capture.format.into();
    let input_channels = ma_device.capture.channels as usize;
    let empty_input_buffer = [0u8; 0];
    let input_frames = if input_ptr.is_null() {
        Frames::wrap(&empty_input_buffer, input_format, input_channels)
    } else {
        Frames::wrap::<u8>(
            std::slice::from_raw_parts(
                input_ptr.cast(),
                input_format.size_in_bytes() * input_channels * frame_count as usize,
            ),
            input_format,
            input_channels,
        )
    };

    let output_format = ma_device.capture.format.into();
    let output_channels = ma_device.capture.channels as usize;
    let mut empty_output_buffer = [0u8; 0];
    let mut output_frames = if output_ptr.is_null() {
        FramesMut::wrap(&mut empty_output_buffer, output_format, output_channels)
    } else {
        FramesMut::wrap::<u8>(
            std::slice::from_raw_parts_mut(
                output_ptr.cast(),
                output_format.size_in_bytes() * input_channels * frame_count as usize,
            ),
            output_format,
            output_channels,
        )
    };

    let device_user_data = &mut *ma_device.pUserData.cast::<DeviceUserData>();
    if let Some(data_callback) = &device_user_data.data_callback {
        let device = device_user_data.device.unwrap();
        let user_data = device_user_data.user_data.as_mut().map(|v| v.as_mut());

        data_callback(device, user_data, &input_frames, &mut output_frames);
    }
}

#[derive(Debug)]
pub struct Device(Box<ma_device>);

impl Device {
    pub fn new(config: DeviceConfig) -> Result<Self, Error> {
        let mut device: Self = {
            let mut device = Box::new(MaybeUninit::<ma_device>::uninit());

            ma_result!(ma_device_init(
                std::ptr::null_mut(),
                &config.0,
                device.as_mut_ptr(),
            ))?;

            unsafe { std::mem::transmute(device) }
        };

        unsafe {
            let raw_config = &mut *(&config.0 as *const _ as *mut ma_device_config);
            let user_data_ptr = std::mem::replace(&mut raw_config.pUserData, std::ptr::null_mut());
            let device_user_data = &mut *user_data_ptr.cast::<DeviceUserData>();

            device_user_data.device = Some(&device);
            device.0.pUserData = device_user_data as *mut _ as _;
        }

        Ok(device)
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
            DeviceType::Playback => self.0.playback.intermediaryBufferCap as _,
            _ => self.0.capture.intermediaryBufferCap as _,
        }
    }

    pub fn start<DataCallback>(&mut self, callback: DataCallback) -> Result<(), Error>
    where
        DataCallback: Fn(&Device, Option<&mut dyn Any>, &Frames, &mut FramesMut) + 'static,
    {
        let device_user_data = unsafe { &mut *self.0.pUserData.cast::<DeviceUserData>() };
        device_user_data.data_callback.replace(Box::new(callback));

        Ok(ma_result!(ma_device_start(self.0.as_mut()))?)
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        Ok(ma_result!(ma_device_stop(self.0.as_mut()))?)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            let user_data_ptr = self.0.pUserData.cast::<DeviceUserData>();
            ma_device_uninit(self.0.as_mut());
            drop(Box::from_raw(user_data_ptr));
        };
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
            let device = Device::new(DeviceConfig::new(
                device_type,
                FORMAT,
                CHANNELS,
                SAMPLE_RATE,
                FRAME_COUNT,
                None,
            ))
            .unwrap();

            assert_eq!(device.device_type(), device_type, "{device_type:?}");
            assert_eq!(device.format(), FORMAT, "{device_type:?}");
            assert_eq!(device.channels(), CHANNELS, "{device_type:?}");
            assert_eq!(device.sample_rate(), SAMPLE_RATE, "{device_type:?}");
            assert_eq!(device.frame_count(), FRAME_COUNT, "{device_type:?}");
        };

        test(DeviceType::Playback);
        test(DeviceType::Capture);
        test(DeviceType::Duplex);

        #[cfg(windows)]
        test(DeviceType::Loopback);
    }

    #[test]
    fn test_user_data() {
        #[derive(Debug, Clone, PartialEq)]
        struct UserData(String);

        let original_user_data = UserData("user data".into());

        let config = DeviceConfig::new(
            DeviceType::Playback,
            FORMAT,
            CHANNELS,
            SAMPLE_RATE,
            FRAME_COUNT,
            Some(Box::new(original_user_data.clone())),
        );

        assert_eq!(
            config.user_data().map(|v| v.downcast_ref::<UserData>()),
            Some(Some(&original_user_data))
        );

        let mut device = Device::new(config).unwrap();

        device
            .start(move |_, user_data, _, _| {
                assert_eq!(
                    user_data.map(|v| v.downcast_ref::<UserData>()),
                    Some(Some(&original_user_data))
                );
            })
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_start_stop() {
        let test = |device_type| {
            let mut device = Device::new(DeviceConfig::new(
                device_type,
                FORMAT,
                CHANNELS,
                SAMPLE_RATE,
                FRAME_COUNT,
                None,
            ))
            .unwrap();

            device
                .start(|device, _, input_frames, output_frames| {
                    let device_type = device.device_type();

                    match device_type {
                        DeviceType::Playback => {
                            assert_eq!(input_frames.frame_count(), 0, "{device_type:?}");
                            assert_eq!(output_frames.frame_count(), FRAME_COUNT, "{device_type:?}");
                        }
                        DeviceType::Capture => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT, "{device_type:?}");
                            assert_eq!(output_frames.frame_count(), 0, "{device_type:?}");
                        }
                        DeviceType::Duplex => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT, "{device_type:?}");
                            assert_eq!(output_frames.frame_count(), FRAME_COUNT, "{device_type:?}");
                        }
                        DeviceType::Loopback => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT, "{device_type:?}");
                            assert_eq!(output_frames.frame_count(), 0, "{device_type:?}");
                        }
                    };
                })
                .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(100));

            device.stop().unwrap();
        };

        test(DeviceType::Playback);
        test(DeviceType::Capture);
        test(DeviceType::Duplex);

        #[cfg(windows)]
        test(DeviceType::Loopback);
    }
}
