mod decoder;
mod encoder;
mod format;
mod frames;
mod miniaudio_error;
mod sample;
mod stream;

pub use decoder::Decoder;
pub use decoder::DecoderConfig;
pub use encoder::Encoder;
pub use encoder::EncoderConfig;
pub use encoder::EncodingFormat;
pub use format::Format;
pub use frames::Frames;
pub use frames::FramesMut;
pub use sample::Sample;
pub use stream::OfflineStream;
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
