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

use std::rc::Rc;
use std::result;

use crate::backend::Backend;
use crate::container::cipher::Cipher;
use crate::container::digest::Digest;
#[cfg(doc)]
use crate::container::error::Error;
use crate::container::kdf::Kdf;
#[cfg(doc)]
use crate::container::Container;
use crate::openssl::OpenSSLError;

#[derive(Debug)]
pub(crate) enum KdfBuilder {
    Pbkdf2(Digest, u32, u32),
    Kdf(Kdf),
}

impl KdfBuilder {
    pub(crate) fn build(&self) -> Result<Kdf, OpenSSLError> {
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
pub struct CreateOptions<B: Backend> {
    pub(crate) backend: B::CreateOptions,
    pub(crate) callback: Option<Rc<dyn Fn() -> result::Result<Vec<u8>, String>>>,
    pub(crate) cipher: Cipher,
    pub(crate) kdf: KdfBuilder,
    pub(crate) top_id: bool,
}

/// Utility used to create a [`CreateOptions`] instance.
pub struct CreateOptionsBuilder<B: Backend>(CreateOptions<B>);

impl<B: Backend> CreateOptionsBuilder<B> {
    /// Creates a builder instance.
    ///
    /// Assigns the [`Backend::CreateOptions`] of the backend to the builder.
    /// The container should use the given `cipher`.
    pub fn new(options: B::CreateOptions, cipher: Cipher) -> Self {
        let kdf = if cipher == Cipher::None {
            KdfBuilder::Kdf(Kdf::None)
        } else {
            KdfBuilder::Pbkdf2(Digest::Sha1, 65536, 16)
        };

        CreateOptionsBuilder(CreateOptions {
            backend: options,
            callback: None,
            cipher,
            kdf,
            top_id: false,
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

    /// If enabled generate a [`Backend::Id`] and store it in the header of the
    /// container.
    ///
    /// This option can be useful if the service running on top of
    /// [`Container`] needs an entrypoint on the container.
    pub fn with_top_id(mut self, enable: bool) -> Self {
        self.0.top_id = enable;
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
    pub fn build(self) -> Result<CreateOptions<B>, B::Err> {
        Ok(self.0)
    }
}

/// Options used to open a container.
///
/// Use the [`OpenOptionsBuilder`] utility to create a `OpenOptions` instance.
pub struct OpenOptions<B: Backend> {
    pub(crate) backend: B::OpenOptions,
    pub(crate) callback: Option<Rc<dyn Fn() -> Result<Vec<u8>, String>>>,
}

/// Utility used to create a [`OpenOptions`] instance.
pub struct OpenOptionsBuilder<B: Backend>(OpenOptions<B>);

impl<B: Backend> OpenOptionsBuilder<B> {
    /// Creates a builder instance using the given [`Backend::OpenOptions`] instance.
    pub fn new(options: B::OpenOptions) -> Self {
        OpenOptionsBuilder(OpenOptions {
            backend: options,
            callback: None,
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

    /// Creates the [`OpenOptions`] instance.
    ///
    /// Before the [`OpenOptions`] instance is created all options passed to
    /// the builder are validated.
    ///
    /// # Errors
    ///
    /// If validation has failed an [`Error`] is returned.
    pub fn build(self) -> Result<OpenOptions<B>, B::Err> {
        Ok(self.0)
    }
}
