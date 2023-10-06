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

use nuts_container::backend::Backend;
use nuts_container::container;
use std::{error, fmt};

pub enum Error<B: Backend> {
    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error occured in a container operation.
    Container(container::Error<B>),

    /// Tried to overwrite an existing
    /// [userdata record](container::Container::userdata).
    OverwriteUserdata,

    /// The [userdata record](container::Container::userdata) of the container
    /// does not refer to an archive:
    ///
    /// * The userdata record is empty, the container has no attached service.
    /// * Another service is attached to the container.
    InvalidUserdata(Option<nuts_bytes::Error>),

    /// Could not parse the header of the archive.
    InvalidHeader(nuts_bytes::Error),
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Error::Container(cause) => fmt::Display::fmt(cause, fmt),
            Error::OverwriteUserdata => write!(fmt, "the container is not empty"),
            Error::InvalidUserdata(None) => write!(fmt, "the container is empty"),
            Error::InvalidUserdata(Some(_)) => {
                write!(fmt, "the container does not have an archive")
            }
            Error::InvalidHeader(_) => write!(fmt, "could not parse the header of the archive"),
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt.debug_tuple("Bytes").field(cause).finish(),
            Self::Container(cause) => fmt.debug_tuple("Container").field(cause).finish(),
            Error::OverwriteUserdata => fmt.debug_tuple("OverwriteUserdata").finish(),
            Self::InvalidUserdata(cause) => {
                fmt.debug_tuple("InvalidUserdata").field(cause).finish()
            }
            Error::InvalidHeader(cause) => fmt.debug_tuple("InvalidHeader").field(cause).finish(),
        }
    }
}

impl<B: Backend> error::Error for Error<B> {}

impl<B: Backend> From<nuts_bytes::Error> for Error<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        match cause {
            nuts_bytes::Error::Serde(ref msg) => {
                if msg == "invalid userdata-magic" {
                    Error::InvalidUserdata(Some(cause))
                } else if msg == "invalid header-magic" {
                    Error::InvalidHeader(cause)
                } else {
                    Error::Bytes(cause)
                }
            }
            _ => Error::Bytes(cause),
        }
    }
}

impl<B: Backend> From<container::Error<B>> for Error<B> {
    fn from(cause: container::Error<B>) -> Self {
        Error::Container(cause)
    }
}

pub type ArchiveResult<T, B> = Result<T, Error<B>>;
