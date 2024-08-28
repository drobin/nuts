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
use crate::migrate::{MigrationError, Migrator};
use crate::options::CreateOptions;
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

pub struct Header<B: Backend> {
    pub(crate) revision: u32,
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Kdf,
    pub(crate) key: SecureVec,
    pub(crate) iv: SecureVec,
    pub(crate) top_id: Option<B::Id>,
}

impl<B: Backend> Header<B> {
    pub fn create(options: &CreateOptions) -> Result<Header<B>, HeaderError> {
        let cipher = options.cipher;
        let mut key = vec![0; cipher.key_len()];
        let mut iv = vec![0; cipher.iv_len()];

        ossl::rand_bytes(&mut key)?;
        ossl::rand_bytes(&mut iv)?;

        let kdf = options.kdf.build()?;

        Ok(Header::<B> {
            revision: 1,
            cipher,
            kdf,
            key: key.into(),
            iv: iv.into(),
            top_id: None,
        })
    }

    pub fn read(
        buf: &[u8],
        migrator: Migrator,
        store: &mut PasswordStore,
    ) -> Result<(Header<B>, B::Settings), HeaderError> {
        match Revision::get_from_buffer(&mut &buf[..])? {
            Revision::Rev0(data) => Self::read_rev0(data, migrator, store),
            Revision::Rev1(data) => Self::read_rev1(data, migrator, store),
        }
    }

    fn read_rev0(
        data: Data,
        migrator: Migrator,
        store: &mut PasswordStore,
    ) -> Result<(Header<B>, B::Settings), HeaderError> {
        let plain_secret =
            data.secret
                .decrypt::<PlainSecretRev0>(store, data.cipher, &data.kdf, &data.iv)?;
        let settings =
            B::Settings::from_bytes(&plain_secret.settings).ok_or(HeaderError::InvalidSettings)?;

        let top_id_vec = migrator.migrate_rev0(&plain_secret.userdata)?;
        let top_id = match top_id_vec {
            Some(vec) => <B::Id as Binary>::from_bytes(&vec),
            None => None,
        };

        Ok((
            Header {
                revision: 0,
                cipher: data.cipher,
                kdf: data.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                top_id,
            },
            settings,
        ))
    }

    fn read_rev1(
        data: Data,
        _migrator: Migrator,
        store: &mut PasswordStore,
    ) -> Result<(Header<B>, B::Settings), HeaderError> {
        let plain_secret =
            data.secret
                .decrypt::<PlainSecretRev1>(store, data.cipher, &data.kdf, &data.iv)?;
        let settings =
            B::Settings::from_bytes(&plain_secret.settings).ok_or(HeaderError::InvalidSettings)?;

        let top_id = match plain_secret.top_id {
            Some(vec) => <B::Id as Binary>::from_bytes(&vec),
            None => None,
        };

        Ok((
            Header {
                revision: 1,
                cipher: data.cipher,
                kdf: data.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                top_id,
            },
            settings,
        ))
    }

    pub fn write(
        &self,
        settings: B::Settings,
        buf: &mut [u8],
        store: &mut PasswordStore,
    ) -> Result<(), HeaderError> {
        let top_id_vec = self
            .top_id
            .as_ref()
            .map(|id| <B::Id as Binary>::as_bytes(id).into());

        let plain_secret = generate_plain_secret(
            self.key.clone(),
            self.iv.clone(),
            top_id_vec,
            settings.as_bytes().into(),
        )?;

        let mut iv = vec![0; self.cipher.iv_len()];
        ossl::rand_bytes(&mut iv)?;

        let secret = plain_secret.encrypt(store, self.cipher, &self.kdf, &iv)?;
        let rev = Revision::latest(self.cipher, iv, self.kdf.clone(), secret);

        rev.put_into_buffer(&mut &mut buf[..])
    }
}

impl<B: Backend> fmt::Debug for Header<B> {
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
            .field("top_id", &self.top_id.as_ref().map(ToString::to_string))
            .finish()
    }
}
