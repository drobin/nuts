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

/// Error type of this library.
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

    /// Cannot aquire another block, the archive is full.
    Full,

    /// The block size of the underlaying [container](container::Container) is
    /// too small.
    InvalidBlockSize,

    /// An error returned by
    /// [`FileEntry::read_all()`](crate::FileEntry::read_all) when the
    /// operation could not be completed because an “end of file” was reached
    /// prematurely.
    UnexpectedEof,

    /// Could not decode the type of an [`Entry`](crate::Entry) stored at the
    /// give block.
    InvalidType(Option<B::Id>),
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::Container(cause) => fmt::Display::fmt(cause, fmt),
            Self::OverwriteUserdata => write!(fmt, "the container is not empty"),
            Self::InvalidUserdata(None) => write!(fmt, "the container is empty"),
            Self::InvalidUserdata(Some(_)) => {
                write!(fmt, "the container does not have an archive")
            }
            Self::InvalidHeader(_) => write!(fmt, "could not parse the header of the archive"),
            Self::Full => write!(fmt, "the archive is full"),
            Self::InvalidBlockSize => write!(fmt, "the block size is too small"),
            Self::UnexpectedEof => write!(fmt, "could not fill the whole buffer"),
            Self::InvalidType(Some(id)) => write!(
                fmt,
                "could not detect the type of the entry stored in {}",
                id
            ),
            Self::InvalidType(None) => {
                write!(
                    fmt,
                    "could not detect the type of the entry in unknown block",
                )
            }
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt.debug_tuple("Bytes").field(cause).finish(),
            Self::Container(cause) => fmt.debug_tuple("Container").field(cause).finish(),
            Self::OverwriteUserdata => fmt.debug_tuple("OverwriteUserdata").finish(),
            Self::InvalidUserdata(cause) => {
                fmt.debug_tuple("InvalidUserdata").field(cause).finish()
            }
            Self::InvalidHeader(cause) => fmt.debug_tuple("InvalidHeader").field(cause).finish(),
            Self::Full => fmt.debug_tuple("Full").finish(),
            Self::InvalidBlockSize => fmt.debug_tuple("InvalidBlockSize").finish(),
            Self::UnexpectedEof => fmt.debug_tuple("UnexpectedEof").finish(),
            Self::InvalidType(id) => fmt.debug_tuple("InvalidType").field(id).finish(),
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
            nuts_bytes::Error::NoSpace(_) | nuts_bytes::Error::Eof(_) => Error::InvalidBlockSize,
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
