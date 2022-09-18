use crate::impl_from_ma_type;
use miniaudio_sys::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown = ma_format_unknown as _,
    U8 = ma_format_u8 as _,
    S16 = ma_format_s16 as _,
    S24 = ma_format_s24 as _,
    S32 = ma_format_s32 as _,
    F32 = ma_format_f32 as _,
}

impl_from_ma_type!(Format, ma_format);

impl Format {
    pub fn size_in_bytes(self) -> usize {
        match self {
            Self::Unknown => 0,
            Self::U8 => 1,
            Self::S16 => 2,
            Self::S24 => 3,
            Self::S32 => 4,
            Self::F32 => 4,
        }
    }
}
