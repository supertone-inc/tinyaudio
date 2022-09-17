use crate::Format;
use crate::Sample;

pub struct Frames<'s> {
    data: &'s [u8],
    format: Format,
    channels: usize,
}

impl<'s> Frames<'s> {
    #[inline]
    pub fn wrap<S: Sample>(data: &'s [S], format: Format, channels: usize) -> Frames<'s> {
        let byte_data_len = data.len() * std::mem::size_of::<S>();
        let byte_data =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_data_len) };

        Frames {
            data: byte_data,
            format,
            channels,
        }
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    #[inline]
    pub fn channels(&self) -> usize {
        self.channels
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    #[inline]
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn as_samples<S: Sample>(&self) -> &[S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn iter<'t, S: 'static + Sample>(&'t self) -> impl 't + Iterator<Item = &[S]> {
        FramesIter {
            samples: self.as_samples(),
            channels: self.channels,
            offset: 0,
        }
    }
}

pub struct FramesMut<'s> {
    data: &'s mut [u8],
    format: Format,
    channels: usize,
}

impl<'s> FramesMut<'s> {
    #[inline]
    pub fn wrap<S: Sample>(data: &'s mut [S], format: Format, channels: usize) -> FramesMut<'s> {
        let byte_data_len = data.len() * std::mem::size_of::<S>();
        let byte_data =
            unsafe { std::slice::from_raw_parts_mut(data.as_ptr() as *mut u8, byte_data_len) };

        FramesMut {
            data: byte_data,
            format,
            channels,
        }
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    #[inline]
    pub fn channels(&self) -> usize {
        self.channels
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    #[inline]
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.data
    }

    #[inline]
    pub fn as_samples<S: Sample>(&self) -> &[S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn as_samples_mut<S: Sample>(&mut self) -> &mut [S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn iter<'t, S: 'static + Sample>(&'t self) -> impl 't + Iterator<Item = &[S]> {
        FramesIter {
            samples: self.as_samples(),
            channels: self.channels,
            offset: 0,
        }
    }

    #[inline]
    pub fn iter_mut<'t, S: 'static + Sample>(&'t mut self) -> impl 't + Iterator<Item = &mut [S]> {
        let channels = self.channels;
        let samples = self.as_samples_mut();
        let samples_len = samples.len();

        FramesIterMut {
            samples_ptr: samples.as_mut_ptr(),
            samples_len,
            channels,
            offset: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct FramesIter<'s, S: Sample> {
    samples: &'s [S],
    channels: usize,
    offset: usize,
}

impl<'s, S: Sample> Iterator for FramesIter<'s, S> {
    type Item = &'s [S];

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.samples.len() {
            let ret = Some(&self.samples[self.offset..(self.offset + self.channels)]);
            self.offset += self.channels;
            ret
        } else {
            None
        }
    }
}

pub struct FramesIterMut<'s, S: Sample> {
    samples_ptr: *mut S,
    samples_len: usize,
    channels: usize,
    offset: usize,
    phantom: std::marker::PhantomData<&'s S>,
}

impl<'s, S: Sample> Iterator for FramesIterMut<'s, S> {
    type Item = &'s mut [S];

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.samples_len {
            let ret = Some(unsafe {
                std::slice::from_raw_parts_mut(self.samples_ptr.add(self.offset), self.channels)
            });
            self.offset += self.channels;
            ret
        } else {
            None
        }
    }
}
