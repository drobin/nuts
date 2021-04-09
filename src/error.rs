// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

use std::{error, fmt};

#[derive(PartialEq, Debug)]
pub(crate) enum InvalHeaderError {
    /// Invalid magic.
    ///
    /// The first few bytes encodes a magic string, which is incorrect.
    InvalMagic,

    /// Invalid revision.
    InvalRevision,

    /// Invalid cipher.
    InvalCipher,

    /// Invalid digest.
    InvalDigest,

    /// Invalid wrapping key.
    InvalWrappingKey,

    /// Invalid IV.
    InvalIv,

    /// Invalid disk type.
    InvalDiskType,

    /// Invalid block size.
    InvalBlockSize,

    /// Invalid number of blocks.
    InvalBlocks,

    /// Invalid master key.
    InvalMasterKey,

    /// Invalid master iv.
    InvalMasterIv,
}

impl fmt::Display for InvalHeaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            InvalHeaderError::InvalMagic => "magic",
            InvalHeaderError::InvalRevision => "revision",
            InvalHeaderError::InvalCipher => "cipher",
            InvalHeaderError::InvalDigest => "digest",
            InvalHeaderError::InvalWrappingKey => "wrapping-key",
            InvalHeaderError::InvalIv => "iv",
            InvalHeaderError::InvalDiskType => "disk-type",
            InvalHeaderError::InvalBlockSize => "block-size",
            InvalHeaderError::InvalBlocks => "blocks",
            InvalHeaderError::InvalMasterKey => "master-key",
            InvalHeaderError::InvalMasterIv => "master-iv",
        };

        write!(fmt, "invalid {} detected", name)
    }
}

impl error::Error for InvalHeaderError {}

/// Collection of error-codes.
#[derive(Debug)]
pub enum Error {
    /// The container is open but it is expected to be closed.
    ///
    /// The error is raised by [`Container::create()`] and
    /// [`Container::open()`] if the container is already open.
    ///
    /// [`Container::create()`]: ../container/struct.Container.html#method.create
    /// [`Container::open()`]: ../container/struct.Container.html#method.open
    Opened,

    /// The container is closed but it is expected to be open.
    ///
    /// You try to perform an operation on a closed [`Container`].
    ///
    /// [`Container`]: ../container/struct.Container.html
    Closed,

    /// An IO error has occured.
    ///
    /// The related [`std::error::Error`] is passed to the variant.
    ///
    /// [`std::error::Error`]: https://doc.rust-lang.org/nightly/std/io/struct.Error.html
    IoError(std::io::Error),

    /// An invalid argument was passed to a function.
    ///
    /// It has a message, that describes the failure.
    InvalArg(String),

    /// Not enough data available to read from a source.
    NoData,

    /// Not enough space available to write into a target.
    NoSpace,

    /// A password is needed by the current cipher.
    NoPassword,

    /// An error occured in the unterlaying OpenSSL library.
    OpenSSL(openssl::error::ErrorStack),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Opened => write!(
                fmt,
                "The container is open but it is expected to be closed."
            ),
            Error::Closed => write!(
                fmt,
                "The container is closed but it is expected to be open."
            ),
            Error::IoError(cause) => write!(fmt, "An IO error occured: {}", cause),
            Error::InvalArg(msg) => write!(fmt, "Invalid argument: {}", msg),
            Error::NoData => write!(fmt, "Not enough data available to read from a source."),
            Error::NoSpace => write!(fmt, "Not enough space available to write into a target."),
            Error::NoPassword => write!(fmt, "A password is needed by the current cipher."),
            Error::OpenSSL(cause) => write!(fmt, "An OpenSSL error occured: {}", cause),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Opened => None,
            Error::Closed => None,
            Error::IoError(ref err) => Some(err),
            Error::InvalArg(_) => None,
            Error::NoData => None,
            Error::NoSpace => None,
            Error::NoPassword => None,
            Error::OpenSSL(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(error: openssl::error::ErrorStack) -> Self {
        Error::OpenSSL(error)
    }
}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::IoError(err) => err,
            _ => {
                let msg = format!("{:?}", error);
                std::io::Error::new(std::io::ErrorKind::Other, msg)
            }
        }
    }
}
