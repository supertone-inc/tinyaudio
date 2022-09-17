use miniaudio_sys::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown = ma_format_unknown,
    U8 = ma_format_u8,
    S16 = ma_format_s16,
    S24 = ma_format_s24,
    S32 = ma_format_s32,
    F32 = ma_format_f32,
}

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

impl From<ma_format> for Format {
    fn from(format: ma_format) -> Self {
        unsafe { std::mem::transmute(format) }
    }
}
