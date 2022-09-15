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

impl From<ma_format> for Format {
    fn from(format: ma_format) -> Self {
        unsafe { std::mem::transmute(format) }
    }
}
