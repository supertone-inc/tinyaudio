mod decoder;
mod device;
mod encoder;
mod format;
mod frames;
mod miniaudio_error;
mod sample;
mod stream;

pub use decoder::Decoder;
pub use decoder::DecoderConfig;
pub use device::Device;
pub use encoder::Encoder;
pub use encoder::EncoderConfig;
pub use encoder::EncodingFormat;
pub use format::Format;
pub use frames::Frames;
pub use frames::FramesMut;
pub use sample::Sample;
pub use stream::CodecStream;
pub use stream::Stream;

#[macro_export]
macro_rules! impl_from_ma_type {
    ($RustType:ty, $CType:ty) => {
        impl From<$CType> for $RustType {
            fn from(c_value: $CType) -> Self {
                unsafe { std::mem::transmute(c_value) }
            }
        }
    };
}

#[macro_export]
macro_rules! ma_result {
    ($Result:expr) => {{
        use crate::miniaudio_error::MiniaudioError;
        use miniaudio_sys::*;
        use std::mem::transmute;

        #[allow(unused_unsafe)]
        unsafe {
            match $Result {
                MA_SUCCESS => Ok(()),
                err => Err(transmute::<ma_result, MiniaudioError>(err)),
            }
        }
    }};
}

#[cfg(not(windows))]
fn path_to_c_string<P: AsRef<std::path::Path>>(path: P) -> std::ffi::CString {
    use std::os::unix::prelude::OsStrExt;

    unsafe { std::ffi::CString::from_vec_unchecked(path.as_ref().as_os_str().as_bytes().into()) }
}

#[cfg(windows)]
fn path_to_c_string<P: AsRef<std::path::Path>>(path: P) -> widestring::WideCString {
    unsafe { widestring::WideCString::from_os_str_unchecked(path.as_ref().as_os_str()) }
}
