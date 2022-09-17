use crate::Frames;
use crate::FramesMut;
use std::error::Error;

pub trait Stream {
    fn start<StreamCallback>(&mut self, callback: StreamCallback) -> Result<(), Box<dyn Error>>
    where
        StreamCallback: Fn(&Frames, &mut FramesMut);

    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
}
