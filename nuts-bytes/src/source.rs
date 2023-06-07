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
    fn take_bytes_to(&mut self, buf: &mut [u8]) -> Result<()> {
        self.take_bytes(buf.len()).map(|bytes| {
            buf.copy_from_slice(bytes.as_ref());
            ()
        })
    }
}

/// `TakeBytes` is implemented for `&[u8]` by taking the first part of the
/// slice.
///
/// Note that taking bytes updates the slice to point to the yet unread part.
/// The slice will be empty when EOF is reached.
impl<'tb> TakeBytes<'tb> for &'tb [u8] {
    fn take_bytes(&mut self, n: usize) -> Result<Cow<'tb, [u8]>> {
        if n <= self.len() {
            let (a, b) = self.split_at(n);

            *self = b;

            Ok(Cow::Borrowed(a))
        } else {
            Err(Error::Eof(None))
        }
    }
}
