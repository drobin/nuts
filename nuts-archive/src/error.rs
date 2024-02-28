// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use nuts_backend::Backend;
use thiserror::Error;

use crate::{header::HeaderMagicError, userdata::UserdataMagicError};

/// Error type of this library.
#[derive(Debug, Error)]
pub enum Error<B: Backend> {
    /// Error while (de-) serializing binary data.
    #[error(transparent)]
    Bytes(nuts_bytes::Error),

    /// An error occured in a container operation.
    #[error(transparent)]
    Container(#[from] nuts_container::Error<B>),

    /// Tried to overwrite an existing
    /// [userdata record](nuts_container::Container::userdata).
    #[error("the container is not empty")]
    OverwriteUserdata,

    /// The [userdata record](nuts_container::Container::userdata) of the
    /// container does not refer to an archive:
    ///
    /// * The userdata record is empty, the container has no attached service.
    /// * Another service is attached to the container.
    #[error("the container {}", if .0.is_some() { "is empty" } else { "does not have an archive" })]
    InvalidUserdata(Option<nuts_bytes::Error>),

    /// Could not parse the header of the archive.
    #[error("could not parse the header of the archive")]
    InvalidHeader(nuts_bytes::Error),

    /// Cannot aquire another block, the archive is full.
    #[error("the archive is full")]
    Full,

    /// The block size of the underlaying
    /// [container](nuts_container::Container) is too small.
    #[error("the block size is too small")]
    InvalidBlockSize,

    /// Tries to read an archive node, which is invalid.
    #[error("not an archive node: {0}")]
    InvalidNode(B::Id),

    /// An error returned by
    /// [`FileEntry::read_all()`](crate::FileEntry::read_all) when the
    /// operation could not be completed because an “end of file” was reached
    /// prematurely.
    #[error("could not fill the whole buffer")]
    UnexpectedEof,

    /// Could not decode the type of an [`Entry`](crate::Entry) stored at the
    /// give block.
    #[error("could not detect the type of the entry {}", if let Some(id) = .0 { format!("stored in {}", id) } else { "in unknown block".to_string() })]
    InvalidType(Option<B::Id>),
}

impl<B: Backend> From<nuts_bytes::Error> for Error<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        match &cause {
            nuts_bytes::Error::Custom(cust) => {
                if cust.is::<HeaderMagicError>() {
                    Error::InvalidHeader(cause)
                } else if cust.is::<UserdataMagicError>() {
                    Error::InvalidUserdata(Some(cause))
                } else {
                    Error::Bytes(cause)
                }
            }
            nuts_bytes::Error::TakeBytes(nuts_bytes::TakeBytesError::Eof)
            | nuts_bytes::Error::PutBytes(nuts_bytes::PutBytesError::NoSpace) => {
                Error::InvalidBlockSize
            }
            _ => Error::Bytes(cause),
        }
    }
}

pub type ArchiveResult<T, B> = Result<T, Error<B>>;
