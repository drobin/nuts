// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use std::mem;
use thiserror::Error;

/// Error type for the [`PutBytes`] trait.
#[derive(Debug, Error, PartialEq)]
pub enum PutBytesError {
    /// No more space available when writing into a byte slice.
    #[error("no more space available for writing")]
    NoSpace,
}

/// Trait that describes a writer of binary data.
///
/// The [`Writer`](crate::Writer) utility accepts all types that implements
/// this trait.
pub trait PutBytes {
    /// Appends all the given data in `buf` at the end of this writer.
    ///
    /// # Errors
    ///
    /// If not all data could be written, the implementator should
    /// return a [`PutBytesError::NoSpace`] error.
    fn put_bytes(&mut self, buf: &[u8]) -> Result<(), PutBytesError>;
}

/// `PutBytes` is implemented for `&mut [u8]` by copying into the slice,
/// overwriting its data.
///
/// Note that putting bytes updates the slice to point to the yet unwritten part.
/// The slice will be empty when it has been completely overwritten.
///
/// If the number of bytes to be written exceeds the size of the slice, the
/// operation will return a [`PutBytesError::NoSpace`] error.
impl PutBytes for &mut [u8] {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<(), PutBytesError> {
        if self.len() >= buf.len() {
            let (a, b) = mem::replace(self, &mut []).split_at_mut(buf.len());

            a.copy_from_slice(buf);
            *self = b;

            Ok(())
        } else {
            Err(PutBytesError::NoSpace)
        }
    }
}

/// `PutBytes` is implemented for [`Vec<u8>`] by appending bytes to the `Vec`.
impl PutBytes for Vec<u8> {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<(), PutBytesError> {
        Ok(self.extend_from_slice(buf))
    }
}

/// `PutBytes` is implemented for [&mut `Vec<u8>`] by appending bytes to the `Vec`.
impl PutBytes for &mut Vec<u8> {
    fn put_bytes(&mut self, buf: &[u8]) -> Result<(), PutBytesError> {
        Ok(self.extend_from_slice(buf))
    }
}
