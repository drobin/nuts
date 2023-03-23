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

use serde::{de, ser};
use std::{error, fmt, result, str::Utf8Error};

/// Integer types from [`Error::InvalidInteger`].
#[derive(Debug, PartialEq)]
pub enum IntType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

/// Errors thrown by the `bytes` modules.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Failed to read the requested number of bytes. No more bytes are
    /// available for reading.
    Eof,

    /// No more space available when writing into a byte slice.
    NoSpace,

    /// Invalid integer type was found.
    ///
    /// The reader tried to read type `expected`, but found type `found`
    /// instead.
    InvalidInteger { expected: IntType, found: IntType },

    /// Tried to deserialize the given `u32` value into a character.
    ///
    /// But the `u32` is not a char.
    InvalidChar(u32),

    /// Failed to deserialize into a string. The source byte data are not valid
    /// UTF-8.
    InvalidString(Utf8Error),

    /// Trailing (not consumed) data were detected.
    TrailingBytes,

    /// The length is unknown when serializing a sequence or map.
    RequiredLength,

    /// A custom error message from Serde.
    Serde(String),
}

impl Error {
    pub(crate) fn invalid_integer(expected: IntType, found: IntType) -> Error {
        Error::InvalidInteger { expected, found }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Eof => write!(fmt, "No more bytes are available for reading."),
            Error::NoSpace => write!(fmt, "no more space available for writing"),
            Error::InvalidInteger { expected, found } => write!(
                fmt,
                "tries to read type {:?}, but found type {:?}",
                expected, found
            ),
            Error::InvalidChar(n) => write!(fmt, "not a char: {}", n),
            Error::InvalidString(cause) => write!(fmt, "not a string: {}", cause),
            Error::TrailingBytes => write!(fmt, "there are trailing bytes available"),
            Error::RequiredLength => write!(fmt, "the length of the sequence or map is required"),
            Error::Serde(msg) => fmt::Display::fmt(msg, fmt),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::InvalidString(cause) => Some(cause),
            _ => None,
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}

/// An [`Error`] wrapping [`Result`](std::result::Result).
pub type Result<T> = result::Result<T, Error>;
