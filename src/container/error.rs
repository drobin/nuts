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
use thiserror::Error as ThisError;

use crate::backend::Backend;
use crate::container::cipher::CipherError;
use crate::container::kdf::KdfError;

#[derive(Debug, ThisError)]
/// Error type used by this module.
pub enum Error<B: Backend> {
    /// An error occured in the attached backend.
    #[error(transparent)]
    Backend(B::Err),

    /// Error while (de-) serializing binary data.
    #[error(transparent)]
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    #[error(transparent)]
    OpenSSL(#[from] ErrorStack),

    /// A cipher related error
    #[error(transparent)]
    Cipher(#[from] CipherError),

    /// A KDF related error
    #[error(transparent)]
    Kdf(#[from] KdfError),

    /// No password callback is assigned to the container, thus no password
    /// is available.
    #[error("a password is needed by the current cipher")]
    NoPassword,

    /// The password callback generated an error, which is passed to the
    /// variant.
    #[error("failed to receive the password: {0}")]
    PasswordCallback(String),

    /// The password is wrong.
    #[error("the password is wrong")]
    WrongPassword(#[source] nuts_bytes::Error),

    /// Try to read/write from/to a null-id which is forbidden.
    #[error("tried to read or write a null id")]
    NullId,
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

pub type ContainerResult<T, B> = Result<T, Error<B>>;
