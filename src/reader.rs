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

use std::marker::PhantomData;
use std::string::FromUtf8Error;
use thiserror::Error;

use crate::from_bytes::{FromBytes, TakeStringError};
use crate::take_bytes::{TakeBytes, TakeBytesError};

/// Default error type of the [`Reader`] utility.
#[derive(Debug, Error)]
pub enum ReaderError {
    /// Failed to read the requested number of bytes. No more bytes are
    /// available for reading.
    #[error("no more bytes are available for reading")]
    Eof,

    /// Failed to deserialize into a string. The source byte data are not valid
    /// UTF-8.
    #[error("the string is invalid: {0}")]
    InvalidString(#[source] FromUtf8Error),
}

impl TakeBytesError for ReaderError {
    fn eof() -> Self {
        Self::Eof
    }
}

impl TakeStringError for ReaderError {
    fn invalid_string(err: FromUtf8Error) -> Self {
        Self::InvalidString(err)
    }
}

/// A cursor like utility that reads structured data from an arbitrary source.
///
/// The source must implement the [`TakeBytes`] trait which supports reading
/// binary data from it.
pub struct Reader<TB, E = ReaderError> {
    source: TB,
    data: PhantomData<E>,
}

impl<TB: TakeBytes, E: TakeBytesError> Reader<TB, E> {
    /// Creates a new `Reader` instance.
    ///
    /// The source of the reader is passed to the function. Every type that
    /// implements the [`TakeBytes`] trait can be the source of this reader.
    pub fn new(source: TB) -> Reader<TB, E> {
        Reader {
            source,
            data: PhantomData,
        }
    }

    /// Deserializes from this binary representation into a data structure
    /// which implements the [`FromBytes`] trait.
    pub fn read<FB: FromBytes<E>>(&mut self) -> Result<FB, E> {
        FromBytes::from_bytes(&mut self.source)
    }
}

impl<TB> AsRef<TB> for Reader<TB> {
    fn as_ref(&self) -> &TB {
        &self.source
    }
}
