use crate::impl_from_ma_type;
use crate::ma_result;
use crate::miniaudio_error::MiniaudioError;
use crate::Format;
use crate::Frames;
use crate::FramesMut;
use miniaudio_sys::*;
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
        let mut config = unsafe { ma_device_config_init(device_type as _) };

        config.sampleRate = sample_rate as _;
        config.periodSizeInFrames = frame_count as _;
        config.dataCallback = Some(device_data_callback);

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

struct DeviceUserData<'a> {
    device: &'a Device,
    data_callback: Box<dyn Fn(&Device, &Frames, &mut FramesMut)>,
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

    let user_data = &*ma_device.pUserData.cast::<DeviceUserData>();
    let device = &user_data.device;
    let data_callback = &user_data.data_callback;
    data_callback(device, &input_frames, &mut output_frames);
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
            DeviceType::Playback => self.0.playback.intermediaryBufferCap as _,
            _ => self.0.capture.intermediaryBufferCap as _,
        }
    }

    pub fn start<DataCallback>(&mut self, callback: DataCallback) -> Result<(), Error>
    where
        DataCallback: Fn(&Device, &Frames, &mut FramesMut) + 'static,
    {
        self.0.pUserData = Box::into_raw(Box::new(DeviceUserData {
            device: &self,
            data_callback: Box::new(callback),
        })) as _;

        Ok(ma_result!(ma_device_start(self.0.as_mut()))?)
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        ma_result!(ma_device_stop(self.0.as_mut()))?;

        let user_data_ptr = std::mem::replace(&mut self.0.pUserData, std::ptr::null_mut());
        drop(unsafe { Box::from_raw(user_data_ptr.cast::<DeviceUserData>()) });

        Ok(())
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

        #[cfg(windows)]
        test(DeviceType::Loopback);
    }

    #[test]
    fn test_start_stop() {
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;
        use std::sync::Arc;

        let test = |device_type| {
            let mut device = Device::new(&DeviceConfig::new(
                device_type,
                FORMAT,
                CHANNELS,
                SAMPLE_RATE,
                FRAME_COUNT,
            ))
            .unwrap();

            let count = Arc::new(AtomicUsize::new(0));
            let count_clone = count.clone();

            device
                .start(move |device, input_frames, output_frames| {
                    count_clone.fetch_add(1, Ordering::Relaxed);

                    match device.device_type() {
                        DeviceType::Playback => {
                            assert_eq!(input_frames.frame_count(), 0);
                            assert_eq!(output_frames.frame_count(), FRAME_COUNT);
                        }
                        DeviceType::Capture => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT);
                            assert_eq!(output_frames.frame_count(), 0);
                        }
                        DeviceType::Duplex => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT);
                            assert_eq!(output_frames.frame_count(), FRAME_COUNT);
                        }
                        DeviceType::Loopback => {
                            assert_eq!(input_frames.frame_count(), FRAME_COUNT);
                            assert_eq!(output_frames.frame_count(), FRAME_COUNT);
                        }
                    };
                })
                .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(10));

            device.stop().unwrap();

            assert!(count.load(Ordering::Relaxed) > 0);
        };

        test(DeviceType::Playback);
        test(DeviceType::Capture);
        test(DeviceType::Duplex);

        #[cfg(windows)]
        test(DeviceType::Loopback);
    }
}
