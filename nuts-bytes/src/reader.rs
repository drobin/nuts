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

#[cfg(feature = "derive")]
use crate::derive::TakeDeriveError;
use crate::from_bytes::{FromBytes, FromBytesError};
use crate::take_bytes::TakeBytes;

#[cfg(feature = "derive")]
impl TakeDeriveError for ReaderError {
    fn invalid_variant_index(idx: usize) -> Self {
        Self::InvalidVariantIndex(idx)
    }
}

/// A cursor like utility that reads structured data from an arbitrary source.
///
/// The source must implement the [`TakeBytes`] trait which supports reading
/// binary data from it.
pub struct Reader<TB> {
    source: TB,
}

impl<TB: TakeBytes> Reader<TB> {
    /// Creates a new `Reader` instance.
    ///
    /// The source of the reader is passed to the function. Every type that
    /// implements the [`TakeBytes`] trait can be the source of this reader.
    pub fn new(source: TB) -> Reader<TB> {
        Reader { source }
    }

    /// Deserializes from this binary representation into a data structure
    /// which implements the [`FromBytes`] trait.
    pub fn read<FB: FromBytes>(&mut self) -> Result<FB, FromBytesError> {
        FromBytes::from_bytes(&mut self.source)
    }
}

impl<TB> AsRef<TB> for Reader<TB> {
    fn as_ref(&self) -> &TB {
        &self.source
    }
}
