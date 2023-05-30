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
use std::str::Utf8Error;
use std::{error, fmt, io, result};

/// Errors thrown by the `bytes` modules.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the requested number of bytes. No more bytes are
    /// available for reading.
    Eof(Option<Box<dyn error::Error + Send + Sync>>),

    /// No more space available when writing into a byte slice.
    NoSpace(Option<Box<dyn error::Error + Send + Sync>>),

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

    /// An I/O error occured.
    Io(io::Error),

    /// A custom error message from Serde.
    Serde(String),

    /// Another custom error occured.
    Other(Box<dyn error::Error + Send + Sync>),
}

impl Error {
    /// Creates a new [`Error::Eof`] error from the given cause.
    pub fn eof<E: Into<Box<dyn error::Error + Send + Sync>>>(err: E) -> Error {
        Error::Eof(Some(err.into()))
    }

    /// Creates a new [`Error::NoSpace`] error from the given cause.
    pub fn nospace<E: Into<Box<dyn error::Error + Send + Sync>>>(err: E) -> Error {
        Error::NoSpace(Some(err.into()))
    }

    /// Creates a new [`Error::Other`] error from the given cause.
    pub fn other<E: Into<Box<dyn error::Error + Send + Sync>>>(err: E) -> Error {
        Error::Other(err.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Eof(_) => write!(fmt, "No more bytes are available for reading."),
            Error::NoSpace(_) => write!(fmt, "no more space available for writing"),
            Error::InvalidChar(n) => write!(fmt, "not a char: {}", n),
            Error::InvalidString(cause) => write!(fmt, "not a string: {}", cause),
            Error::TrailingBytes => write!(fmt, "there are trailing bytes available"),
            Error::RequiredLength => write!(fmt, "the length of the sequence or map is required"),
            Error::Io(cause) => fmt::Display::fmt(cause, fmt),
            Error::Serde(msg) => fmt::Display::fmt(msg, fmt),
            Error::Other(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Eof(Some(cause)) => Some(cause.as_ref()),
            Error::NoSpace(Some(cause)) => Some(cause.as_ref()),
            Error::InvalidString(cause) => Some(cause),
            Error::Io(cause) => Some(cause),
            Error::Other(cause) => Some(cause.as_ref()),
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

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Self {
        match cause.kind() {
            io::ErrorKind::UnexpectedEof => Error::Eof(Some(cause.into())),
            io::ErrorKind::WriteZero => Error::NoSpace(Some(cause.into())),
            _ => Error::Io(cause),
        }
    }
}

/// An [`Error`] wrapping [`Result`](std::result::Result).
pub type Result<T> = result::Result<T, Error>;
