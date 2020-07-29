// MIT License
//
// Copyright (c) 2020 Robin Doer
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

/// Details about an [`Error::InvalHeader`] error.
///
/// [`Error::InvalHeader`]: enum.Error.html#variant.InvalHeader
#[derive(PartialEq, Debug)]
pub enum InvalHeaderKind {
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

    /// Invalid HMAC.
    InvalHmac,

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

    /// Invalid HMAC key.
    InvalHmacKey,
}

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

    /// An invalid header of the container was detected.
    ///
    /// The value contains some details about the error.
    InvalHeader(InvalHeaderKind),

    /// Not enough data available to read from a source.
    NoData,

    /// Not enough space available to write into a target.
    NoSpace,

    /// A password is needed by the current cipher.
    NoPassword,

    /// An error occured in the unterlaying OpenSSL library.
    OpenSSL(openssl::error::ErrorStack),

    /// An hmac mismatch was detected.
    HmacMismatch,
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
