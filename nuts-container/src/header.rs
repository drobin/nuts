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
#[cfg(test)]
mod tests;

use nuts_backend::{Backend, Binary};
use openssl::error::ErrorStack;
use plain_secret::PlainSecret;
use std::borrow::Cow;
use std::fmt;
use std::ops::DerefMut;
use thiserror::Error;

use crate::buffer::{BufferError, ToBuffer};
use crate::cipher::{Cipher, CipherContext, CipherError};
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

    /// Invalid top-id, could not parse top-id from header.
    #[error("invalid top-id")]
    InvalidTopId,

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

pub struct Header<'a, B: Backend> {
    revision: u32,
    migrator: Migrator<'a>,
    cipher: Cipher,
    kdf: Kdf,
    data: PlainSecret<B>,
}

impl<'a, B: Backend> Header<'a, B> {
    pub fn create(
        options: &CreateOptions,
        settings: B::Settings,
    ) -> Result<Header<'a, B>, HeaderError> {
        let cipher = options.cipher;
        let mut key = vec![0; cipher.key_len()];
        let mut iv = vec![0; cipher.iv_len()];

        ossl::rand_bytes(&mut key)?;
        ossl::rand_bytes(&mut iv)?;

        let kdf = options.kdf.build()?;
        let plain_secret = PlainSecret::create_latest(key.into(), iv.into(), settings)?;

        Ok(Header {
            revision: 1,
            migrator: Migrator::default(),
            cipher,
            kdf,
            data: plain_secret,
        })
    }

    pub fn read(
        buf: &[u8],
        migrator: Migrator<'a>,
        store: &mut PasswordStore,
    ) -> Result<Header<'a, B>, HeaderError> {
        match Revision::get_from_buffer(&mut &buf[..])? {
            Revision::Rev0(data) => Self::read_rev0(data, migrator, store),
            Revision::Rev1(data) => Self::read_rev1(data, migrator, store),
        }
    }

    fn read_rev0(
        data: Data,
        migrator: Migrator<'a>,
        store: &mut PasswordStore,
    ) -> Result<Header<'a, B>, HeaderError> {
        let key = Self::create_key(data.cipher, &data.kdf, store)?;
        let mut ctx = Self::prepare_cipher_ctx(data.cipher, &data.secret);

        let pbuf = ctx.decrypt(&key, &data.iv)?;
        let plain_secret = PlainSecret::from_buffer_rev0(&mut &pbuf[..])?;

        Ok(Header {
            revision: 0,
            migrator,
            cipher: data.cipher,
            kdf: data.kdf,
            data: plain_secret,
        })
    }

    fn read_rev1(
        data: Data,
        migrator: Migrator<'a>,
        store: &mut PasswordStore,
    ) -> Result<Header<'a, B>, HeaderError> {
        let key = Self::create_key(data.cipher, &data.kdf, store)?;
        let mut ctx = Self::prepare_cipher_ctx(data.cipher, &data.secret);

        let pbuf = ctx.decrypt(&key, &data.iv)?;
        let plain_secret = PlainSecret::from_buffer_rev1(&mut &pbuf[..])?;

        Ok(Header {
            revision: 1,
            migrator,
            cipher: data.cipher,
            kdf: data.kdf,
            data: plain_secret,
        })
    }

    pub fn write(&self, buf: &mut [u8], store: &mut PasswordStore) -> Result<(), HeaderError> {
        let mut iv = vec![0; self.cipher.iv_len()];
        ossl::rand_bytes(&mut iv)?;

        let mut pbuf: SecureVec = vec![].into();
        self.data.to_buffer(pbuf.deref_mut())?;

        let key = Self::create_key(self.cipher, &self.kdf, store)?;
        let mut ctx = Self::prepare_cipher_ctx(self.cipher, &pbuf);

        let cbuf = ctx.encrypt(&key, &iv)?;
        let secret = cbuf.to_vec();

        let rev = match self.data {
            PlainSecret::Rev0(_) => Revision::new_rev0(self.cipher, iv, self.kdf.clone(), secret),
            PlainSecret::Rev1(_) => Revision::new_rev1(self.cipher, iv, self.kdf.clone(), secret),
        };

        rev.put_into_buffer(&mut &mut buf[..])
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn cipher(&self) -> Cipher {
        self.cipher
    }

    pub fn kdf(&self) -> &Kdf {
        &self.kdf
    }

    pub fn settings(&self) -> &B::Settings {
        match &self.data {
            PlainSecret::Rev0(rev0) => &rev0.settings,
            PlainSecret::Rev1(rev1) => &rev1.settings,
        }
    }

    pub fn key(&self) -> &[u8] {
        match &self.data {
            PlainSecret::Rev0(rev0) => &rev0.key,
            PlainSecret::Rev1(rev1) => &rev1.key,
        }
    }

    pub fn iv(&self) -> &[u8] {
        match &self.data {
            PlainSecret::Rev0(rev0) => &rev0.iv,
            PlainSecret::Rev1(rev1) => &rev1.iv,
        }
    }

    pub fn top_id(&'a self) -> Result<Option<Cow<'a, B::Id>>, HeaderError> {
        match &self.data {
            PlainSecret::Rev0(rev0) => match self.migrator.migrate_rev0(&rev0.userdata)? {
                Some(vec) => match <B::Id as Binary>::from_bytes(&vec) {
                    Some(id) => Ok(Some(Cow::Owned(id))),
                    None => Err(HeaderError::InvalidTopId),
                },
                None => Ok(None),
            },
            PlainSecret::Rev1(rev1) => Ok(rev1.top_id.as_ref().map(Cow::Borrowed)),
        }
    }

    pub fn set_top_id(&mut self, id: B::Id) {
        match &mut self.data {
            PlainSecret::Rev0(_) => panic!("storing a top-id into a rev0 header is not supported"),
            PlainSecret::Rev1(rev1) => rev1.top_id = Some(id),
        }
    }

    fn prepare_cipher_ctx(cipher: Cipher, input: &[u8]) -> CipherContext {
        let mut ctx = CipherContext::new(cipher);

        ctx.copy_from_slice(input.len(), input);

        ctx
    }

    fn create_key(
        cipher: Cipher,
        kdf: &Kdf,
        store: &mut PasswordStore,
    ) -> Result<SecureVec, HeaderError> {
        if cipher.key_len() > 0 {
            let password = store.value()?;
            Ok(kdf.create_key(password, cipher.key_len())?)
        } else {
            Ok(vec![].into())
        }
    }
}

impl<'a, B: Backend> fmt::Debug for Header<'a, B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Header")
            .field("revision", &self.revision)
            .field("migrator", &self.migrator)
            .field("cipher", &self.cipher)
            .field("kdf", &self.kdf)
            .field("data", &self.data)
            .finish()
    }
}
