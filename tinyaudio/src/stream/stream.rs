use crate::Frames;
use crate::FramesMut;

pub trait Stream {
    type Error;

    fn start<StreamCallback>(&mut self, callback: StreamCallback) -> Result<(), Self::Error>
    where
        StreamCallback: Fn(&Frames, &mut FramesMut) + 'static;

    fn stop(&mut self) -> Result<(), Self::Error>;
}
