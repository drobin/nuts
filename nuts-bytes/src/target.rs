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

use std::io::Write;

use crate::error::{Error, Result};
#[cfg(doc)]
use crate::writer::Writer;

/// Trait that describes a writer of binary data.
///
/// The [`Writer`] utility accepts all types that implements this trait.
pub trait PutBytes {
    /// Appends all the given data in `buf` at the end of this writer.
    ///
    /// # Errors
    ///
    /// If not all data could be written, an [`Error::NoSpace`] error should be
    /// returned.
    fn put_bytes(&mut self, buf: &[u8]) -> Result<()>;
}

/// A writer target that writes into a slice of binary data.
///
/// Data are appended to the slice but an [`Error::NoSpace`] error is generated
/// if the end of the slice is reached.
///
/// # Example
///
/// ```rust
/// use nuts_bytes::BufferTarget;
///
/// let mut buf = [0; 64];
/// let target = BufferTarget::new(&mut buf);
/// ```
pub struct BufferTarget<'a> {
    buf: &'a mut [u8],
    offs: usize,
}

impl<'a> BufferTarget<'a> {
    /// Creates a new `BufferTarget` instance which writes into the given `buf`.
    pub fn new(buf: &'a mut [u8]) -> BufferTarget<'a> {
        BufferTarget { buf, offs: 0 }
    }

    /// Returns the current position in the buffer.
    pub fn position(&self) -> usize {
        self.offs
    }
}

impl<'a> PutBytes for BufferTarget<'a> {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<()> {
        match self.buf.get_mut(self.offs..self.offs + buf.len()) {
            Some(target) => {
                target.copy_from_slice(buf);
                self.offs += buf.len();
                Ok(())
            }
            None => Err(Error::NoSpace(None)),
        }
    }
}

impl<'a> AsRef<[u8]> for BufferTarget<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

/// A writer target that writes into a [`Vec<u8>`].
///
/// The vector grows automatically when appending data.
///
/// # Example
///
/// ```rust
/// use nuts_bytes::VecTarget;
///
/// let target = VecTarget::new(vec![]);
/// ```
pub struct VecTarget(Vec<u8>);

impl VecTarget {
    /// Creates a new `VecTarget` instance which writes into the given `vec`.
    pub fn new(vec: Vec<u8>) -> VecTarget {
        VecTarget(vec)
    }

    /// Consumes this `VecTarget`, returning the underlying [`Vec`].
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl PutBytes for VecTarget {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<()> {
        Ok(self.0.extend_from_slice(buf))
    }
}

impl AsRef<[u8]> for VecTarget {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// A writer target that writes into a [`Write`] instance.
///
/// Writes into a stream type that implements the [`Write`] trait.
///
/// # Example
///
/// ```rust
/// use nuts_bytes::StreamTarget;
/// use std::io::Cursor;
///
/// let cursor = Cursor::<Vec<u8>>::new(vec![]);
/// let target = StreamTarget::new(cursor);
/// ```
pub struct StreamTarget<W>(W);

impl<W> StreamTarget<W> {
    /// Creates a new `StreamTarget` instance which writes into the given `target`.
    pub fn new(target: W) -> StreamTarget<W> {
        StreamTarget(target)
    }

    /// Consumes this `StreamTarget`, returning the underlying value.
    pub fn into_target(self) -> W {
        self.0
    }
}

impl<W: Write> PutBytes for StreamTarget<W> {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<()> {
        Ok(self.0.write_all(buf)?)
    }
}

impl<W> AsRef<W> for StreamTarget<W> {
    fn as_ref(&self) -> &W {
        &self.0
    }
}
