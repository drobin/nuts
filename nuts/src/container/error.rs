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

use std::{error, fmt, result};

use crate::backend::Backend;
use crate::openssl::OpenSSLError;

/// Error type used by this module.
pub enum ContainerError<B: Backend> {
    /// An error occured in the attached backend.
    Backend(B::Err),

    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    OpenSSL(OpenSSLError),

    /// A password is needed by the current cipher.
    NoPassword(Option<String>),

    /// The password is wrong.
    WrongPassword,

    /// Try to read/write from/to a null-id which is forbidden.
    NullId,
}

impl<B: Backend> fmt::Display for ContainerError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerError::Backend(cause) => fmt::Display::fmt(cause, fmt),
            ContainerError::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            ContainerError::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            ContainerError::NoPassword(Some(msg)) => {
                write!(fmt, "A password is needed by the current cipher: {}", msg)
            }
            ContainerError::NoPassword(None) => {
                write!(fmt, "A password is needed by the current cipher")
            }
            ContainerError::WrongPassword => write!(fmt, "The password is wrong."),
            ContainerError::NullId => write!(fmt, "Try to read or write a null id"),
        }
    }
}

impl<B: Backend> fmt::Debug for ContainerError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerError::Backend(cause) => fmt::Debug::fmt(cause, fmt),
            ContainerError::Bytes(cause) => fmt::Debug::fmt(cause, fmt),
            ContainerError::OpenSSL(cause) => fmt::Debug::fmt(cause, fmt),
            ContainerError::NoPassword(option) => fmt::Debug::fmt(option, fmt),
            ContainerError::WrongPassword | ContainerError::NullId => write!(fmt, "{:?}", self),
        }
    }
}

impl<B: Backend + 'static> error::Error for ContainerError<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ContainerError::Backend(cause) => Some(cause),
            ContainerError::Bytes(cause) => Some(cause),
            ContainerError::OpenSSL(cause) => Some(cause),
            ContainerError::NoPassword(_)
            | ContainerError::WrongPassword
            | ContainerError::NullId => None,
        }
    }
}

impl<B: Backend> From<nuts_bytes::Error> for ContainerError<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        ContainerError::Bytes(cause)
    }
}

impl<B: Backend> From<OpenSSLError> for ContainerError<B> {
    fn from(cause: OpenSSLError) -> Self {
        ContainerError::OpenSSL(cause)
    }
}

pub type ContainerResult<T, B> = result::Result<T, ContainerError<B>>;
