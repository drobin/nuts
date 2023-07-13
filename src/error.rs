// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use std::{error, fmt, io, result};

/// The error type for the directory backend.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occured.
    Io(io::Error),

    /// You are creating a new backend which already exists.
    Exists,

    /// Could not generate a unique [id](crate::Id).
    UniqueId,

    /// The [id](crate::Id) is invalid, is not a hex string.
    InvalidId(String),

    /// The block size passed to [CreateOptions](crate::CreateOptions) is
    /// invalid.
    InvalidBlockSize(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(cause) => fmt::Display::fmt(cause, fmt),
            Error::Exists => write!(fmt, "The container already exists"),
            Error::UniqueId => write!(fmt, "could not generate a unique id"),
            Error::InvalidId(id) => write!(fmt, "The id '{}' is invalid", id),
            Error::InvalidBlockSize(n) => write!(fmt, "The block-size is invalid: {}", n),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(cause) => Some(cause),
            Error::Exists | Error::UniqueId | Error::InvalidId(_) | Error::InvalidBlockSize(_) => {
                None
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Self {
        Error::Io(cause)
    }
}

pub type Result<T> = result::Result<T, Error>;
