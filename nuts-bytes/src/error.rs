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

use std::string::FromUtf8Error;
use thiserror::Error;

use crate::put_bytes::PutBytesError;
use crate::take_bytes::TakeBytesError;

/// Error type of the library.
#[derive(Debug, Error)]
pub enum Error {
    /// Errors coming from [`TakeBytes`](crate::take_bytes::TakeBytes).
    #[error(transparent)]
    TakeBytes(#[from] TakeBytesError),

    /// Errors coming from [`PutBytes`](crate::put_bytes::PutBytes).
    #[error(transparent)]
    PutBytes(#[from] PutBytesError),

    /// Failed to deserialize into a `char`. The source `u32` cannot be
    /// converted into a `char`.
    #[error("the char is invalid, {0} is not a char")]
    InvalidChar(u32),

    /// Failed to deserialize into a string. The source byte data are not valid
    /// UTF-8.
    #[error("the string is invalid: {0}")]
    InvalidString(#[source] FromUtf8Error),

    /// A custom error occured.
    #[error("{0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),

    /// Deserialized an invalid variant index.
    /// There is no enum variant at the given index.
    #[cfg(feature = "derive")]
    #[error("invalid enum, no variant at {0}")]
    InvalidVariantIndex(u32),
}
