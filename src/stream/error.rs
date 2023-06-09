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

use std::{error, fmt};

use crate::backend::Backend;
use crate::container;
#[cfg(doc)]
use crate::stream::Stream;

/// Error type used by the module.
pub enum Error<B: Backend> {
    /// Failed to encoding/decode the content of a block.
    Bytes(nuts_bytes::Error),

    /// Failed to read/write data from/to the container.
    Container(container::Error<B>),

    /// The [`Stream::read_all()`] function was not able to completely fill the
    /// buffer.
    ReadAll,

    /// The [`Stream::write_all()`] function was not able to write the whole
    /// buffer.
    WriteAll,

    /// The read operation is not allowed on this stream.
    NotReadable,

    /// The write operation is not allowed on this stream.
    NotWritable,
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::Container(cause) => fmt::Display::fmt(cause, fmt),
            Self::ReadAll => write!(fmt, "failed to fill whole buffer"),
            Self::WriteAll => write!(fmt, "failed to write whole buffer"),
            Self::NotReadable => write!(fmt, "You are not allowed to read the stream."),
            Self::NotWritable => write!(fmt, "You are not allowed to write the stream."),
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt.debug_tuple("Bytes").field(cause).finish(),
            Self::Container(cause) => fmt.debug_tuple("Container").field(cause).finish(),
            Self::ReadAll => write!(fmt, "ReadAll"),
            Self::WriteAll => write!(fmt, "WriteAll"),
            Self::NotReadable => write!(fmt, "NotReadable"),
            Self::NotWritable => write!(fmt, "NotWritable"),
        }
    }
}

impl<B: Backend> error::Error for Error<B> {}

impl<B: Backend> From<container::Error<B>> for Error<B> {
    fn from(cause: container::Error<B>) -> Self {
        Error::Container(cause)
    }
}

impl<B: Backend> From<nuts_bytes::Error> for Error<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        Error::Bytes(cause)
    }
}
