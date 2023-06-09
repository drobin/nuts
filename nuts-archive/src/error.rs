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

use std::{error, fmt, io, result};

use nuts::backend::Backend;
use nuts::stream;
#[cfg(doc)]
use nuts::{
    container::{Container, CreateOptionsBuilder},
    stream::Stream,
};

#[cfg(doc)]
use crate::builder::Builder;

/// Error type of this crate.
pub enum Error<B: Backend> {
    /// The associated [`Container`] has no top-id.
    ///
    /// Make sure the container is created with the [`CreateOptionsBuilder::with_top_id()`] option.
    NoTopId,

    /// An I/O error has occured.
    Io(io::Error),

    /// An error occured in the internal [`Stream`].
    Stream(stream::Error<B>),

    /// Failed to (de-)serialize data.
    Bytes(nuts_bytes::Error),

    /// Error returned by a [`Builder`] of an archive entry.
    Builder(Box<dyn error::Error + Send + Sync>),
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoTopId => write!(fmt, "NoTopId"),
            Self::Io(cause) => fmt.debug_tuple("Io").field(cause).finish(),
            Self::Stream(cause) => fmt.debug_tuple("Stream").field(cause).finish(),
            Self::Bytes(cause) => fmt.debug_tuple("Bytes").field(cause).finish(),
            Self::Builder(cause) => fmt.debug_tuple("Builder").field(cause).finish(),
        }
    }
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoTopId => write!(fmt, "no top-id enabled for the container"),
            Error::Io(cause) => fmt::Display::fmt(cause, fmt),
            Error::Stream(cause) => fmt::Display::fmt(cause, fmt),
            Error::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Error::Builder(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl<B: 'static + Backend> error::Error for Error<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NoTopId => None,
            Self::Io(cause) => Some(cause),
            Self::Stream(cause) => Some(cause),
            Self::Bytes(cause) => Some(cause),
            Self::Builder(cause) => Some(cause.as_ref()),
        }
    }
}

impl<B: Backend> From<io::Error> for Error<B> {
    fn from(cause: io::Error) -> Self {
        Error::Io(cause)
    }
}

impl<B: Backend> From<stream::Error<B>> for Error<B> {
    fn from(cause: stream::Error<B>) -> Self {
        Error::Stream(cause)
    }
}

impl<B: Backend> From<nuts_bytes::Error> for Error<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        Error::Bytes(cause)
    }
}

/// The [`Result`](result::Result) type of this crate.
pub type Result<T, B> = result::Result<T, Error<B>>;
