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

use crate::error::Error;
use crate::put_bytes::PutBytes;
use crate::to_bytes::ToBytes;

/// A cursor like utility that writes structured data into an arbitrary target.
///
/// The target must implement the [`PutBytes`] trait which supports writing
/// binary data into it.
#[derive(Debug)]
pub struct Writer<T> {
    target: T,
}

impl<T: PutBytes> Writer<T> {
    /// Creates a new `Writer` instance.
    ///
    /// The target, where the writer puts the binary data, is passed to the
    /// function. Every type, that implements the [`PutBytes`] trait can be the
    /// target of this writer.
    pub fn new(target: T) -> Writer<T> {
        Writer { target }
    }

    /// Serializes a data structure that implements the [`ToBytes`] trait.
    ///
    /// Returns the number of bytes actually serialized.
    pub fn write<TB: ToBytes>(&mut self, value: &TB) -> Result<usize, Error> {
        ToBytes::to_bytes(value, &mut self.target)
    }

    /// Consumes this `Writer`, returning the underlying target.
    pub fn into_target(self) -> T {
        self.target
    }
}

impl<T> AsRef<T> for Writer<T> {
    fn as_ref(&self) -> &T {
        &self.target
    }
}
