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

mod plain_secret;
mod revision;
mod secret;
#[cfg(test)]
mod tests;

use nuts_backend::{Backend, Binary};
use openssl::error::ErrorStack;
use std::fmt::{self, Write as FmtWrite};
use thiserror::Error;

use crate::buffer::BufferError;
use crate::cipher::{Cipher, CipherError};
use crate::header::plain_secret::generate_plain_secret;
use crate::header::plain_secret::{Encryptor, PlainSecretRev0, PlainSecretRev1};
use crate::header::revision::{Data, Revision};
use crate::kdf::{Kdf, KdfError};
use crate::migrate::MigrationError;
use crate::options::{CreateOptions, OpenOptions};
use crate::ossl;
use crate::password::{PasswordError, PasswordStore};
use crate::svec::SecureVec;

/// Header related errors.
#[derive(Debug, Error)]
pub enum HeaderError {
    /// A cipher related error
    #[error(transparent)]
    Cipher(#[from] CipherError),

    /// A KDF related error
    #[error(transparent)]
    Kdf(#[from] KdfError),

    /// A password related error.
    #[error(transparent)]
    Password(#[from] PasswordError),

    /// The password is wrong.
    #[error("the password is wrong")]
    WrongPassword,

    /// Invalid header, could not validate magic
    #[error("invalid header")]
    InvalidHeader,

    /// Invalid settings, could not parse backend settings from header.
    #[error("invalid settings")]
    InvalidSettings,

    /// Error while (de-) serializing binary data.
    #[error(transparent)]
    Buffer(#[from] BufferError),

    /// An error in the OpenSSL library occured.
    #[error(transparent)]
    OpenSSL(#[from] ErrorStack),

    /// Errors coming from a migration
    #[error(transparent)]
    Migration(#[from] MigrationError),
}

pub struct Header {
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Kdf,
    pub(crate) key: SecureVec,
    pub(crate) iv: SecureVec,
    pub(crate) top_id: Option<SecureVec>,
}

impl Header {
    pub fn create(options: &CreateOptions) -> Result<Header, HeaderError> {
        let cipher = options.cipher;
        let mut key = vec![0; cipher.key_len()];
        let mut iv = vec![0; cipher.iv_len()];

        ossl::rand_bytes(&mut key)?;
        ossl::rand_bytes(&mut iv)?;

        let kdf = options.kdf.build()?;

        Ok(Header {
            cipher,
            kdf,
            key: key.into(),
            iv: iv.into(),
            top_id: None,
        })
    }

    pub fn read<B: Backend>(
        buf: &[u8],
        options: OpenOptions,
        store: &mut PasswordStore,
    ) -> Result<(Header, B::Settings), HeaderError> {
        match Revision::get_from_buffer(&mut &buf[..])? {
            Revision::Rev0(data) => Self::read_rev0::<B>(data, options, store),
            Revision::Rev1(data) => Self::read_rev1::<B>(data, options, store),
        }
    }

    fn read_rev0<B: Backend>(
        data: Data,
        options: OpenOptions,
        store: &mut PasswordStore,
    ) -> Result<(Header, B::Settings), HeaderError> {
        let plain_secret =
            data.secret
                .decrypt::<PlainSecretRev0>(store, data.cipher, &data.kdf, &data.iv)?;
        let settings =
            B::Settings::from_bytes(&plain_secret.settings).ok_or(HeaderError::InvalidSettings)?;

        let top_id = options.migrator.migrate_rev0(&plain_secret.userdata)?;

        Ok((
            Header {
                cipher: data.cipher,
                kdf: data.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                top_id,
            },
            settings,
        ))
    }

    fn read_rev1<B: Backend>(
        data: Data,
        _options: OpenOptions,
        store: &mut PasswordStore,
    ) -> Result<(Header, B::Settings), HeaderError> {
        let plain_secret =
            data.secret
                .decrypt::<PlainSecretRev1>(store, data.cipher, &data.kdf, &data.iv)?;
        let settings =
            B::Settings::from_bytes(&plain_secret.settings).ok_or(HeaderError::InvalidSettings)?;

        Ok((
            Header {
                cipher: data.cipher,
                kdf: data.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                top_id: plain_secret.top_id,
            },
            settings,
        ))
    }

    pub fn write<B: Backend>(
        &self,
        settings: B::Settings,
        buf: &mut [u8],
        store: &mut PasswordStore,
    ) -> Result<(), HeaderError> {
        let plain_secret = generate_plain_secret(
            self.key.clone(),
            self.iv.clone(),
            self.top_id.clone(),
            settings.as_bytes().into(),
        )?;

        let mut iv = vec![0; self.cipher.iv_len()];
        ossl::rand_bytes(&mut iv)?;

        let secret = plain_secret.encrypt(store, self.cipher, &self.kdf, &iv)?;
        let rev = Revision::latest(self.cipher, iv, self.kdf.clone(), secret);

        rev.put_into_buffer(&mut &mut buf[..])
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (key, iv) = if cfg!(feature = "debug-plain-keys") {
            let mut key = String::with_capacity(2 * self.key.len());
            let mut iv = String::with_capacity(2 * self.iv.len());

            for n in self.key.iter() {
                write!(key, "{:02x}", n)?;
            }

            for n in self.iv.iter() {
                write!(iv, "{:02x}", n)?;
            }

            (key, iv)
        } else {
            (
                format!("<{} bytes>", self.key.len()),
                format!("<{} bytes>", self.iv.len()),
            )
        };

        fmt.debug_struct("Header")
            .field("cipher", &self.cipher)
            .field("kdf", &self.kdf)
            .field("key", &key)
            .field("iv", &iv)
            .finish()
    }
}
