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
#[cfg(doc)]
use crate::container::cipher::Cipher;
use crate::container::password::PasswordError;

/// Error type used by this module.
pub enum Error<B: Backend> {
    /// An error occured in the attached backend.
    Backend(B::Err),

    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    OpenSSL(ErrorStack),

    /// The cipher key is invalid/too short.
    InvalidKey,

    /// The cipher iv is invalid/too short.
    InvalidIv,

    /// The size of the block to be encrypted/decrypted is invalid and must be
    /// aligned at the [block size](Cipher::block_size) of the cipher.
    InvalidBlockSize,

    /// A cipher-text is not trustworthy.
    ///
    /// If an authenticated decryption is performed, and the tag mismatches,
    /// this error is raised.
    NotTrustworthy,

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
            Self::Backend(cause) => fmt::Display::fmt(cause, fmt),
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Self::InvalidKey => write!(fmt, "Invalid key"),
            Self::InvalidIv => write!(fmt, "Invalid iv"),
            Self::InvalidBlockSize => write!(fmt, "Invalid block-size"),
            Self::NotTrustworthy => write!(fmt, "The plaintext is not trustworthy"),
            Self::Password(cause) => fmt::Display::fmt(cause, fmt),
            Self::WrongPassword(_) => write!(fmt, "The password is wrong."),
            Self::NullId => write!(fmt, "Try to read or write a null id"),
        }
    }
}

impl<B: Backend> fmt::Debug for Error<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Backend(cause) => fmt::Debug::fmt(cause, fmt),
            Self::Bytes(cause) => fmt::Debug::fmt(cause, fmt),
            Self::OpenSSL(cause) => fmt::Debug::fmt(cause, fmt),
            Self::InvalidKey => fmt.write_str("InvalidKey"),
            Self::InvalidIv => fmt.write_str("InvalidIv"),
            Self::InvalidBlockSize => fmt.write_str("InvalidBlockSize"),
            Self::NotTrustworthy => fmt.write_str("NotTrustworthy"),
            Self::Password(cause) => fmt::Debug::fmt(cause, fmt),
            Self::WrongPassword(cause) => fmt::Debug::fmt(cause, fmt),
            Self::NullId => fmt.write_str("NullId"),
        }
    }
}

impl<B: Backend + 'static> error::Error for Error<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Backend(cause) => Some(cause),
            Self::Bytes(cause) => Some(cause),
            Self::OpenSSL(cause) => Some(cause),
            Self::InvalidKey => None,
            Self::InvalidIv => None,
            Self::InvalidBlockSize => None,
            Self::NotTrustworthy => None,
            Self::Password(cause) => Some(cause),
            Self::WrongPassword(cause) => Some(cause),
            Self::NullId => None,
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

impl<B: Backend> From<PasswordError> for Error<B> {
    fn from(cause: PasswordError) -> Self {
        Error::Password(cause)
    }
}

pub type ContainerResult<T, B> = Result<T, Error<B>>;
