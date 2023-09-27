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

use openssl::error::ErrorStack;
use std::{error, fmt};

use crate::backend::Backend;
use crate::container::cipher::CipherError;
use crate::container::password::PasswordError;

/// Error type used by this module.
pub enum Error<B: Backend> {
    /// An error occured in the attached backend.
    Backend(B::Err),

    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    OpenSSL(ErrorStack),

    /// An error occured in a cipher operation.
    Cipher(CipherError),

    /// A password is needed by the current cipher.
    Password(PasswordError),

    /// The password is wrong.
    WrongPassword(nuts_bytes::Error),

    /// Try to read/write from/to a null-id which is forbidden.
    NullId,
}

impl<B: Backend> fmt::Display for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Backend(cause) => fmt::Display::fmt(cause, fmt),
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Error::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Error::Cipher(cause) => fmt::Display::fmt(cause, fmt),
            Self::Password(cause) => fmt::Display::fmt(cause, fmt),
            Self::WrongPassword(_) => write!(fmt, "The password is wrong."),
            Error::NullId => write!(fmt, "Try to read or write a null id"),
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Backend(cause) => fmt::Debug::fmt(cause, fmt),
            Error::Bytes(cause) => fmt::Debug::fmt(cause, fmt),
            Error::OpenSSL(cause) => fmt::Debug::fmt(cause, fmt),
            Error::Cipher(cause) => fmt::Debug::fmt(cause, fmt),
            Error::Password(cause) => fmt::Debug::fmt(cause, fmt),
            Error::WrongPassword(cause) => fmt::Debug::fmt(cause, fmt),
            Error::NullId => fmt.write_str("NullId"),
        }
    }
}

impl<B: Backend + 'static> error::Error for Error<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Backend(cause) => Some(cause),
            Error::Bytes(cause) => Some(cause),
            Error::OpenSSL(cause) => Some(cause),
            Error::Cipher(cause) => Some(cause),
            Error::Password(cause) => Some(cause),
            Error::WrongPassword(cause) => Some(cause),
            Error::NullId => None,
        }
    }
}

impl<B: Backend> From<nuts_bytes::Error> for Error<B> {
    fn from(cause: nuts_bytes::Error) -> Self {
        match &cause {
            nuts_bytes::Error::Serde(msg) => {
                if msg == "secret-magic mismatch" {
                    return Error::WrongPassword(cause);
                }
            }
            _ => {}
        }

        Error::Bytes(cause)
    }
}

impl<B: Backend> From<ErrorStack> for Error<B> {
    fn from(cause: ErrorStack) -> Self {
        Error::OpenSSL(cause)
    }
}

impl<B: Backend> From<CipherError> for Error<B> {
    fn from(cause: CipherError) -> Self {
        Error::Cipher(cause)
    }
}

impl<B: Backend> From<PasswordError> for Error<B> {
    fn from(cause: PasswordError) -> Self {
        Error::Password(cause)
    }
}

pub type ContainerResult<T, B> = Result<T, Error<B>>;
