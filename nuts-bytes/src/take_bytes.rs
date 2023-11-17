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

/// Error type for the [`TakeBytes`] trait.
///
/// [`TakeBytes`] can generate one error: Not enough data are available in the
/// source. This kind of error can be created with [`TakeBytesError::eof()`].
pub trait TakeBytesError: std::error::Error {
    /// Creates an EOF error.
    ///
    /// This error should be raised if the source has not anough data
    /// available.
    fn eof() -> Self;
}

/// Trait that describes a reader of binary data.
///
/// The [`Reader`](crate::Reader) utility accepts all types that implements
/// this trait.
pub trait TakeBytes {
    /// Reads some bytes from the source and puts them into the given buffer
    /// `buf`.
    ///
    /// # Errors
    ///
    /// If not enough data are available to fill `buf` the implementator should
    /// create an EOF error with [`TakeBytesError::eof()`].
    fn take_bytes<E: TakeBytesError>(&mut self, buf: &mut [u8]) -> Result<(), E>;
}

/// `TakeBytes` is implemented for `&[u8]` by taking the first part of the
/// slice.
///
/// Note that taking bytes updates the slice to point to the yet unread part.
/// The slice will be empty when EOF is reached.
impl TakeBytes for &[u8] {
    fn take_bytes<E: TakeBytesError>(&mut self, buf: &mut [u8]) -> Result<(), E> {
        if buf.len() <= self.len() {
            let (a, b) = self.split_at(buf.len());

            *self = b;
            buf.copy_from_slice(a);

            Ok(())
        } else {
            Err(E::eof())
        }
    }
}
