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

use std::borrow::Cow;
use std::cmp;
use std::io::Read;

use crate::error::{Error, Result};
#[cfg(doc)]
use crate::reader::Reader;

/// Trait that describes a reader of binary data.
///
/// The [`Reader`] utility accepts all types that implements this trait.
pub trait TakeBytes<'tb> {
    /// Reads `n` bytes from the source.
    ///
    /// If possible a slice of borrowed data of the given size (`n`) wrapped
    /// into [`Cow::Borrowed`] should be returned.
    ///
    /// If the data cannot be borrowed a [`Vec<u8>`] wrapped into a
    /// [`Cow::Owned`] should be returned.
    ///
    /// # Errors
    ///
    /// If not enough data are available an [`Error::Eof`] error is returned.
    fn take_bytes(&mut self, n: usize) -> Result<Cow<'tb, [u8]>>;

    /// Reads some bytes from the source and puts them into the given buffer
    /// `buf`.
    ///
    /// # Errors
    ///
    /// If not enough data are available to fill `buf` an [`Error::Eof`] error
    /// is returned.
    fn take_bytes_to(&mut self, buf: &mut [u8]) -> Result<()>;
}

/// A reader source that reads from a slice of binary data.
///
/// # Example
///
/// ```rust
/// use nuts_bytes::BufferSource;
///
/// let source = BufferSource::new(&[1, 2, 3]);
/// ```
pub struct BufferSource<'a> {
    buf: &'a [u8],
    offs: usize,
}

impl<'a> BufferSource<'a> {
    /// Creates a new `BufferSource` instance which reads from the given `buf`.
    pub fn new(buf: &'a [u8]) -> BufferSource<'a> {
        BufferSource { buf, offs: 0 }
    }

    /// Returns the current position in the source.
    pub fn position(&self) -> usize {
        self.offs
    }

    /// Returns the slice of remaining (unread) data from the source.
    ///
    /// If all data were consumed the returned slice is empty.
    pub fn remaining_bytes(&self) -> &'a [u8] {
        let n = cmp::min(self.offs, self.buf.len());
        self.buf.get(n..).unwrap()
    }
}

impl<'tb, 'a: 'tb> TakeBytes<'tb> for BufferSource<'a> {
    fn take_bytes(&mut self, n: usize) -> Result<Cow<'a, [u8]>> {
        match self.buf.get(self.offs..self.offs + n) {
            Some(buf) => {
                self.offs += n;
                Ok(Cow::Borrowed(buf))
            }
            None => Err(Error::Eof(None)),
        }
    }

    fn take_bytes_to(&mut self, buf: &mut [u8]) -> Result<()> {
        self.take_bytes(buf.len()).map(|bytes| {
            buf.copy_from_slice(bytes.as_ref());
            ()
        })
    }
}

/// A reader source that reads from a [`Read`] instance.
///
/// Reads from a stream type that implements the [`Read`] trait.
///
/// # Example
///
/// ```rust
/// use nuts_bytes::StreamSource;
/// use std::io::Cursor;
///
/// let cursor = Cursor::new(&[1, 2, 3]);
/// let source = StreamSource::new(cursor);
/// ```
pub struct StreamSource<R>(R);

impl<R> StreamSource<R> {
    pub fn new(r: R) -> StreamSource<R> {
        StreamSource(r)
    }
}

impl<R> AsRef<R> for StreamSource<R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<'tb, R: Read> TakeBytes<'tb> for StreamSource<R> {
    fn take_bytes(&mut self, n: usize) -> Result<Cow<'tb, [u8]>> {
        let mut vec = Cow::<[u8]>::Owned(vec![0; n]);

        self.0
            .read_exact(vec.to_mut())
            .map_or_else(|err| Err(err.into()), |()| Ok(vec))
    }

    fn take_bytes_to(&mut self, buf: &mut [u8]) -> Result<()> {
        self.0.read_exact(buf).map_err(|err| err.into())
    }
}
