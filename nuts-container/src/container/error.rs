// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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
use thiserror::Error as ThisError;

use crate::container::cipher::CipherError;
use crate::container::header::HeaderError;

/// Error type used by this module.
#[derive(Debug, ThisError)]
pub enum Error<B: Backend> {
    /// An error occured in the attached backend.
    #[error(transparent)]
    Backend(B::Err),

    /// A cipher related error
    #[error(transparent)]
    Cipher(#[from] CipherError),

    /// Errors coming from header evaluation.
    #[error(transparent)]
    Header(#[from] HeaderError),

    /// Try to read/write from/to a null-id which is forbidden.
    #[error("tried to read or write a null id")]
    NullId,
}

pub type ContainerResult<T, B> = Result<T, Error<B>>;
