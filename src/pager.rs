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

#[cfg(test)]
mod tests;

use nuts_bytes::{Reader, Writer};
use nuts_container::{backend::Backend, container::Container};
use std::ops::{Deref, DerefMut};

use crate::error::ArchiveResult;

pub struct BufContainer<B: Backend> {
    inner: Container<B>,
    buf: Vec<u8>,
}

impl<B: Backend> BufContainer<B> {
    pub fn new(container: Container<B>) -> BufContainer<B> {
        let buf = vec![0; container.block_size() as usize];

        BufContainer {
            inner: container,
            buf,
        }
    }

    pub fn create_reader(&self) -> Reader<&[u8]> {
        Reader::new(self.buf.as_slice())
    }

    pub fn create_writer(&mut self) -> Writer<&mut [u8]> {
        self.whiteout();

        Writer::new(self.buf.as_mut_slice())
    }

    pub fn read_buf(&mut self, id: &B::Id) -> ArchiveResult<Reader<&[u8]>, B> {
        self.read_buf_raw(id)?;
        Ok(self.create_reader())
    }

    pub fn read_buf_raw(&mut self, id: &B::Id) -> ArchiveResult<&[u8], B> {
        let n = self.inner.read(id, &mut self.buf)?;

        assert_eq!(n, self.buf.len());

        Ok(&self.buf)
    }

    pub fn write_buf(&mut self, id: &B::Id) -> ArchiveResult<(), B> {
        self.inner.write(id, &self.buf)?;
        Ok(())
    }

    fn whiteout(&mut self) {
        self.buf.iter_mut().for_each(|n| *n = 0)
    }

    pub fn into_container(self) -> Container<B> {
        self.inner
    }
}

impl<B: Backend> Deref for BufContainer<B> {
    type Target = Container<B>;

    fn deref(&self) -> &Container<B> {
        &self.inner
    }
}

impl<B: Backend> DerefMut for BufContainer<B> {
    fn deref_mut(&mut self) -> &mut Container<B> {
        &mut self.inner
    }
}
