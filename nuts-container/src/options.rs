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
use std::rc::Rc;
use std::result;

use crate::cipher::Cipher;
use crate::digest::Digest;
use crate::error::ContainerResult;
use crate::kdf::{Kdf, KdfError};
#[cfg(doc)]
use crate::{error::Error, Container};

#[derive(Debug)]
pub(crate) enum KdfBuilder {
    Pbkdf2(Digest, u32, u32),
    Kdf(Kdf),
}

impl KdfBuilder {
    pub(crate) fn build(&self) -> Result<Kdf, KdfError> {
        match self {
            KdfBuilder::Pbkdf2(digest, iterations, salt_len) => {
                Kdf::generate_pbkdf2(*digest, *iterations, *salt_len)
            }
            KdfBuilder::Kdf(ref kdf) => Ok(kdf.clone()),
        }
    }
}

/// Options used to create a new container.
///
/// Use the [`CreateOptionsBuilder`] utility to create a `CreateOptions`
/// instance.
pub struct CreateOptions {
    pub(crate) callback: Option<Rc<dyn Fn() -> result::Result<Vec<u8>, String>>>,
    pub(crate) cipher: Cipher,
    pub(crate) kdf: KdfBuilder,
    pub(crate) overwrite: bool,
}

/// Utility used to create a [`CreateOptions`] instance.
pub struct CreateOptionsBuilder(CreateOptions);

impl CreateOptionsBuilder {
    /// Creates a builder instance.
    ///
    /// The container should use the given `cipher`.
    pub fn new(cipher: Cipher) -> Self {
        let kdf = if cipher == Cipher::None {
            KdfBuilder::Kdf(Kdf::None)
        } else {
            KdfBuilder::Pbkdf2(Digest::Sha1, 65536, 16)
        };

        CreateOptionsBuilder(CreateOptions {
            callback: None,
            cipher,
            kdf,
            overwrite: false,
        })
    }

    /// Assigns a password callback to the container.
    ///
    /// A password is needed, when encryption is enabled for the container.
    /// Based on the password a wrapping key is generated, which encrypts the
    /// secret part of the header. If encryption is enabled but no password
    /// callback is assigned, an [`Error::NoPassword`] error is raised. If
    /// encryption is disabled, no password is needed and an assigned callback
    /// is never called.
    ///
    /// On success the callback returns the password (represented as an
    /// [`Vec<u8>`](`Vec`)) wrapped into an [`Ok`](`Result::Ok`). On any
    /// failure an [`Err`](`Result::Err`) with an error message must be
    /// returned.
    ///
    /// [`Error::NoPassword`]: enum.Error.html#variant.NoPassword
    pub fn with_password_callback<Cb: Fn() -> Result<Vec<u8>, String> + 'static>(
        mut self,
        callback: Cb,
    ) -> Self {
        self.0.callback = Some(Rc::new(callback));
        self
    }

    /// Uses the given key derivation function.
    ///
    /// If the cipher is set to [`Cipher::None`], then the setting is
    /// discarded;  you don't need a KDF for the None cipher.
    pub fn with_kdf(mut self, kdf: Kdf) -> Self {
        if self.0.cipher != Cipher::None {
            self.0.kdf = KdfBuilder::Kdf(kdf);
        }

        self
    }

    /// Assigns a new overwrite flag to the options.
    ///
    /// If set to `true` an already existing backend is overwritten. If
    /// `overwrite` is set to `false` and the requested container exists, the
    ///  build should fail.
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.0.overwrite = overwrite;
        self
    }

    /// Creates the [`CreateOptions`] instance.
    ///
    /// Before the [`CreateOptions`] instance is created all options passed to
    /// the builder are validated.
    ///
    /// # Errors
    ///
    /// If validation has failed an [`Error`] is returned.
    pub fn build<B: Backend>(self) -> ContainerResult<CreateOptions, B> {
        Ok(self.0)
    }
}

/// Options used to open a container.
///
/// Use the [`OpenOptionsBuilder`] utility to create a `OpenOptions` instance.
pub struct OpenOptions {
    pub(crate) callback: Option<Rc<dyn Fn() -> Result<Vec<u8>, String>>>,
}

/// Utility used to create a [`OpenOptions`] instance.
pub struct OpenOptionsBuilder(OpenOptions);

impl OpenOptionsBuilder {
    /// Creates a builder instance.
    pub fn new() -> Self {
        OpenOptionsBuilder(OpenOptions { callback: None })
    }

    /// Assigns a password callback to the container.
    ///
    /// A password is needed, when encryption is enabled for the container.
    /// Based on the password a wrapping key is generated, which encrypts the
    /// secret part of the header. If encryption is enabled but no password
    /// callback is assigned, an [`Error::NoPassword`] error is raised. If
    /// encryption is disabled, no password is needed and an assigned callback
    /// is never called.
    ///
    /// On success the callback returns the password (represented as an
    /// [`Vec<u8>`](`Vec`)) wrapped into an [`Ok`](`Result::Ok`). On any
    /// failure an [`Err`](`Result::Err`) with an error message must be
    /// returned.
    ///
    /// [`Error::NoPassword`]: enum.Error.html#variant.NoPassword
    pub fn with_password_callback<Cb: Fn() -> Result<Vec<u8>, String> + 'static>(
        mut self,
        callback: Cb,
    ) -> Self {
        self.0.callback = Some(Rc::new(callback));
        self
    }

    /// Creates the [`OpenOptions`] instance.
    ///
    /// Before the [`OpenOptions`] instance is created all options passed to
    /// the builder are validated.
    ///
    /// # Errors
    ///
    /// If validation has failed an [`Error`] is returned.
    pub fn build<B: Backend>(self) -> ContainerResult<OpenOptions, B> {
        Ok(self.0)
    }
}
