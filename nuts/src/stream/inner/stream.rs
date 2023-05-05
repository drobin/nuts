// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use std::cmp;
use std::ops::Deref;

use nuts_backend::Backend;

use crate::container::Container;
use crate::stream::error::Error;
use crate::stream::inner::{EncodeOption, Inner};

fn remaining<B: Backend>(inner: &Inner<B>) -> usize {
    inner.cur.as_ref().map_or(0, |cur| cur.len - cur.offs)
}

macro_rules! cur {
    ($stream:expr) => {
        $stream.cur.as_ref().unwrap()
    };

    (mut $stream:expr) => {
        $stream.cur.as_mut().unwrap()
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    /// Sets the offset to the provided number of bytes.
    Start(u64),

    /// Sets the offset to the size of this object plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it’s an error
    /// to seek before byte 0.
    End(i64),

    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it’s an error
    /// to seek before byte 0.
    Current(i64),
}

/// Options which can be used to configure how a stream is opened.
#[derive(Clone, Copy, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
}

impl OpenOptions {
    /// Creates a new `OpenOptions` builder.
    ///
    /// All options are set to `false`.
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
        }
    }

    /// Sets the option for read access.
    ///
    /// This option, when `true`, will indicate that the stream should be
    /// readable if opened.
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when `true`, will indicate that the stream should be
    /// writeable if opened.
    ///
    /// Any write calls on the stream will overwrite its contents, without
    /// truncating it.
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets the option for the append mode.
    ///
    /// This option, when `true`, means that writes will append to a stream
    /// instead of overwriting previous contents. Note that setting
    /// `.write(true).append(true)` has the same effect as setting only
    /// `.append(true)`.
    ///
    /// **Note**: This function doesn’t create the stream if it doesn’t exist.
    /// Use [`Self::create`] to do so.
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    /// Sets the option for truncating the stream.
    ///
    /// If a file is successfully opened with this option set it will truncate
    /// the stream to 0 length.
    ///
    /// The stream must be opened with write access for truncate to work.
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    /// Sets the option to create a new stream.
    ///
    /// **Note:** An already exising stream will be overwritten!
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    /// Opens a stream from the `container` starting at `id` with the options
    /// specified by `self`.
    pub fn open<B: Backend>(
        self,
        container: Container<B>,
        id: B::Id,
    ) -> Result<Stream<B>, Error<B>> {
        let inner = if self.create {
            Inner::create(container, id)?
        } else {
            Inner::open(container, id)?
        };

        let readable = self.read;
        let writable = self.write || self.append;
        let truncate = writable && self.truncate;

        let mut stream = Stream {
            inner,
            readable,
            writable,
            truncate,
        };

        if self.append {
            stream.seek(Position::End(0))?;
        }

        Ok(stream)
    }
}

pub struct Stream<B: Backend> {
    inner: Inner<B>,
    readable: bool,
    writable: bool,
    truncate: bool,
}

impl<B: Backend> Stream<B> {
    /// Attempts to open a stream in readonly mode.
    ///
    /// This is a convenient method around [`OpenOptions`] to open a stream in
    /// readonly mode. See [`OpenOptions`] for more options.
    pub fn open(container: Container<B>, id: B::Id) -> Result<Stream<B>, Error<B>> {
        OpenOptions::new().read(true).open(container, id)
    }

    /// Creates a stream in writeonly mode.
    ///
    /// This is a convenient method around [`OpenOptions`] to create a new
    /// stream in writeonly mode. See [`OpenOptions`] for more options.
    pub fn create(container: Container<B>, id: B::Id) -> Result<Stream<B>, Error<B>> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(container, id)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error<B>> {
        if !self.readable {
            return Err(Error::NotReadable);
        }

        if buf.len() == 0 {
            return Ok(0);
        }

        while remaining(self) == 0 {
            match self.inner.goto_next() {
                Some(result) => {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                None => return Ok(0),
            }
        }

        let remaining = remaining(self);
        let nbytes = cmp::min(remaining, buf.len());

        let cur = self.inner.cur.as_mut().unwrap();
        let source = &self.inner.buf[cur.start + cur.offs..cur.start + cur.offs + nbytes];
        let target = &mut buf[..nbytes];

        target.copy_from_slice(source);
        cur.offs += nbytes;

        Ok(nbytes)
    }

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function reads as many bytes as necessary to completely fill the
    /// specified buffer `buf`.
    ///
    /// # Errors
    ///
    /// If this function encounters an "end of file" before completely filling
    /// the buffer, it returns an [`Error::ReadAll`] error. The contents of
    /// `buf` are unspecified in this case.
    pub fn read_all(&mut self, buf: &mut [u8]) -> Result<(), Error<B>> {
        let mut nread = 0;

        while nread < buf.len() {
            match self.read(&mut buf[nread..]) {
                Ok(0) => break,
                Ok(n) => nread += n,
                Err(err) => return Err(err),
            };
        }

        if nread == buf.len() {
            Ok(())
        } else {
            Err(Error::ReadAll)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error<B>> {
        if !self.writable {
            return Err(Error::NotWritable);
        }

        if self.inner.current().is_none() {
            self.inner.goto_first()?;

            if self.truncate {
                self.truncate()?;
            }
        }

        if buf.is_empty() {
            return Ok(0);
        }

        let mut nbytes = self.write_overwrite(buf)?;

        if nbytes == 0 && self.is_last() {
            nbytes = self.write_append(buf)?;
        }

        if nbytes == 0 {
            self.flush()?;

            match self.inner.goto_next() {
                Some(Ok(_id)) => {}
                Some(Err(err)) => return Err(err),
                None => {
                    self.inner.insert_back()?;
                }
            };

            self.write(buf)
        } else {
            Ok(nbytes)
        }
    }

    /// Attempts to write an entire buffer into this stream.
    ///
    /// This method will continuously call [`write()`](Self::write) until there
    /// is no more data to be written or an error is returned. This method will
    /// not return until the entire buffer has been successfully written or an
    /// error occurs. The first error from this method will be returned.
    ///
    /// # Errors
    ///
    /// If this function cannot write the whole buffer, it returns an
    /// [`Error::WriteAll`] error.
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), Error<B>> {
        let mut nwritten = 0;

        while nwritten < buf.len() {
            match self.write(&buf[nwritten..]) {
                Ok(0) => break,
                Ok(n) => nwritten += n,
                Err(err) => {
                    return Err(err);
                }
            };
        }

        if nwritten == buf.len() {
            self.flush()
        } else {
            Err(Error::WriteAll)
        }
    }

    fn write_overwrite(&mut self, buf: &[u8]) -> Result<usize, Error<B>> {
        match self.inner.cur.as_mut() {
            Some(cur) => {
                let target = self
                    .inner
                    .buf
                    .get_mut(cur.start + cur.offs..cur.start + cur.len)
                    .unwrap_or_else(|| &mut []);
                let nbytes = cmp::min(target.len(), buf.len());

                target[..nbytes].copy_from_slice(&buf[..nbytes]);
                cur.offs += nbytes;

                Ok(nbytes)
            }
            None => Ok(0),
        }
    }

    fn write_append(&mut self, buf: &[u8]) -> Result<usize, Error<B>> {
        match self.inner.cur.as_mut() {
            Some(cur) => {
                let target = &mut self.inner.buf[cur.start + cur.len..];
                let nbytes = cmp::min(target.len(), buf.len());

                target[..nbytes].copy_from_slice(&buf[..nbytes]);
                cur.len += nbytes;
                cur.offs += nbytes;

                Ok(nbytes)
            }
            None => Ok(0),
        }
    }

    pub fn flush(&mut self) -> Result<(), Error<B>> {
        match self.inner.cur.as_ref() {
            Some(cur) => {
                self.inner
                    .buf
                    .encode_block(&cur.prev, &cur.next, EncodeOption::Skip(cur.len))?;
                self.inner.buf.write(&cur.id).map(|_| ())
            }
            None => Ok(()),
        }
    }

    fn truncate(&mut self) -> Result<(), Error<B>> {
        loop {
            match self.inner.goto_next() {
                Some(Ok(id)) => {
                    let cloned = id.clone();
                    self.inner.buf.container.release(cloned)?;
                }
                Some(Err(err)) => return Err(err),
                None => break,
            }
        }

        self.inner.goto_first()?;

        if let Some(cur) = self.inner.cur.as_mut() {
            cur.prev = cur.id.clone();
            cur.len = 0;

            self.inner.last = cur.id.clone();
        }

        Ok(())
    }

    pub fn seek(&mut self, pos: Position) -> Result<(), Error<B>> {
        let n = match pos {
            Position::Start(n) => self.inner.goto_first().map(|_| n as i64)?,
            Position::End(n) => {
                self.inner.goto_last()?;
                cur!(mut self.inner).offs = cur!(self.inner).len;
                n
            }
            Position::Current(n) => n,
        };

        if n > 0 {
            self.walk_forward(n as u64)?;
        } else if n < 0 {
            let n = if n == i64::MIN {
                2u64.pow(63)
            } else {
                n.abs() as u64
            };

            self.walk_backward(n)?;
        }

        Ok(())
    }

    fn walk_forward(&mut self, nbytes: u64) -> Result<(), Error<B>> {
        let mut remain = nbytes;

        loop {
            let n = cmp::min(remaining(&self.inner) as u64, remain);
            cur!(mut self.inner).offs += n as usize;

            remain -= n;

            if remain > 0 {
                match self.inner.goto_next() {
                    Some(Ok(_)) => {}
                    Some(Err(err)) => return Err(err),
                    None => break,
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn walk_backward(&mut self, nbytes: u64) -> Result<(), Error<B>> {
        let mut remain = nbytes;

        loop {
            let n = cmp::min(cur!(self.inner).offs as u64, remain);
            cur!(mut self.inner).offs -= n as usize;

            remain -= n;

            if remain > 0 {
                match self.inner.goto_prev() {
                    Some(Ok(_)) => {
                        cur!(mut self.inner).offs = cur!(self.inner).len;
                    }
                    Some(Err(err)) => return Err(err),
                    None => break,
                }
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl<B: Backend> AsRef<Inner<B>> for Stream<B> {
    fn as_ref(&self) -> &Inner<B> {
        &self.inner
    }
}

impl<B: Backend> Deref for Stream<B> {
    type Target = Inner<B>;

    fn deref(&self) -> &Inner<B> {
        &self.inner
    }
}
