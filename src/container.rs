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

use nuts_bytes::{Reader, Writer};
use nuts_container::{backend::Backend, container::Container};
use std::ops::{Deref, DerefMut};

use crate::error::ArchiveResult;

pub struct BufWriter<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    writer: Writer<Vec<u8>>,
}

impl<'a, B: Backend> BufWriter<'a, B> {
    fn new(container: &'a mut BufContainer<B>) -> BufWriter<'a, B> {
        let writer = Writer::new(vec![]);

        BufWriter { container, writer }
    }

    pub fn flush(&mut self, id: &B::Id) -> ArchiveResult<(), B> {
        let n = self.container.inner.write(id, &self.writer.as_ref())?;

        assert!(n <= self.container.block_size() as usize);

        Ok(())
    }
}

impl<'a, B: Backend> Deref for BufWriter<'a, B> {
    type Target = Writer<Vec<u8>>;

    fn deref(&self) -> &Writer<Vec<u8>> {
        &self.writer
    }
}

impl<'a, B: Backend> DerefMut for BufWriter<'a, B> {
    fn deref_mut(&mut self) -> &mut Writer<Vec<u8>> {
        &mut self.writer
    }
}

pub struct BufContainer<B: Backend> {
    inner: Container<B>,
    read_buf: Vec<u8>,
}

impl<B: Backend> BufContainer<B> {
    pub fn new(container: Container<B>) -> BufContainer<B> {
        BufContainer {
            inner: container,
            read_buf: vec![],
        }
    }

    pub fn read(&mut self, id: &B::Id) -> ArchiveResult<&[u8], B> {
        self.read_buf.resize(self.inner.block_size() as usize, 0);

        let n = self.inner.read(id, &mut self.read_buf)?;

        assert_eq!(n, self.read_buf.len());

        Ok(&self.read_buf)
    }

    pub fn write(&mut self, id: &B::Id, buf: &[u8]) -> ArchiveResult<(), B> {
        Ok(self.inner.write(id, buf).map(|_| ())?)
    }

    pub fn read_reader(&mut self, id: &B::Id) -> ArchiveResult<Reader<&[u8]>, B> {
        self.read(id)?;

        Ok(Reader::new(self.read_buf.as_slice()))
    }

    pub fn write_writer(&mut self) -> BufWriter<B> {
        BufWriter::new(self)
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
