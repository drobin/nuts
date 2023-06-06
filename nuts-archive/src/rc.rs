// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use nuts::stream::Stream;
use nuts_backend::Backend;
use nuts_bytes::{PutBytes, TakeBytes};
use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct StreamRc<B: Backend>(Rc<RefCell<Stream<B>>>);

impl<B: Backend> StreamRc<B> {
    pub fn new(stream: Stream<B>) -> StreamRc<B> {
        StreamRc(Rc::new(RefCell::new(stream)))
    }
}

impl<'tb, B: 'static + Backend> TakeBytes<'tb> for &mut StreamRc<B> {
    fn take_bytes(&mut self, n: usize) -> nuts_bytes::Result<std::borrow::Cow<'tb, [u8]>> {
        let mut cow = Cow::<[u8]>::Owned(vec![0; n]);

        self.take_bytes_to(cow.to_mut()).map(|()| cow)
    }

    fn take_bytes_to(&mut self, buf: &mut [u8]) -> nuts_bytes::Result<()> {
        self.0.borrow_mut().read_all(buf).map_err(|err| match err {
            nuts::stream::Error::ReadAll => nuts_bytes::Error::eof(err),
            _ => nuts_bytes::Error::other(err),
        })
    }
}

impl<B: 'static + Backend> PutBytes for &mut StreamRc<B> {
    fn put_bytes(&mut self, buf: &[u8]) -> nuts_bytes::Result<()> {
        self.0.borrow_mut().write_all(buf).map_err(|err| match err {
            nuts::stream::Error::WriteAll => nuts_bytes::Error::nospace(err),
            _ => nuts_bytes::Error::other(err),
        })
    }
}

impl<B: Backend> Clone for StreamRc<B> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<B: Backend> Deref for StreamRc<B> {
    type Target = Rc<RefCell<Stream<B>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Backend> DerefMut for StreamRc<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
