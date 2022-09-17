use crate::Format;

pub trait Sample {
    fn format() -> Format;
}

impl Sample for u8 {
    fn format() -> Format {
        Format::U8
    }
}

impl Sample for i16 {
    fn format() -> Format {
        Format::S16
    }
}

impl Sample for [u8; 3] {
    fn format() -> Format {
        Format::S24
    }
}

impl Sample for i32 {
    fn format() -> Format {
        Format::S32
    }
}

impl Sample for f32 {
    fn format() -> Format {
        Format::F32
    }
}
