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

use nuts_backend::Backend;

use crate::container::header::HeaderError;
use crate::openssl::OpenSSLError;

/// Error type used by this module.
pub enum Error<B: Backend> {
    /// An error occured in the attached backend.
    Backend(B::Err),

    /// An error in the OpenSSL library occured.
    OpenSSL(OpenSSLError),

    /// An error occured while evaluating the header of the container.
    Header(HeaderError),

    /// Try to read/write from/to a null-id which is forbidden.
    NullId,
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Backend(cause) => fmt::Display::fmt(cause, fmt),
            Error::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Error::Header(cause) => fmt::Display::fmt(cause, fmt),
            Error::NullId => write!(fmt, "Try to read or write a null id"),
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Backend(cause) => fmt::Debug::fmt(cause, fmt),
            Error::OpenSSL(cause) => fmt::Debug::fmt(cause, fmt),
            Error::Header(cause) => fmt::Debug::fmt(cause, fmt),
            Error::NullId => fmt.write_str("NullId"),
        }
    }
}

impl<B: Backend + 'static> error::Error for Error<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Backend(cause) => Some(cause),
            Error::OpenSSL(cause) => Some(cause),
            Error::Header(cause) => Some(cause),
            Error::NullId => None,
        }
    }
}

impl<B: Backend> From<OpenSSLError> for Error<B> {
    fn from(cause: OpenSSLError) -> Self {
        Error::OpenSSL(cause)
    }
}

impl<B: Backend> From<HeaderError> for Error<B> {
    fn from(cause: HeaderError) -> Self {
        Error::Header(cause)
    }
}

pub type ContainerResult<T, B> = Result<T, Error<B>>;
