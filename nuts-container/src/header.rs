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

use log::{debug, error};
use nuts_backend::Backend;
use openssl::error::ErrorStack;
use plain_secret::PlainSecret;
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

pub const LATEST_REVISION: u32 = 2;

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

    /// Invalid header revision
    #[error("invalid header revision, expected {0} but got {1}")]
    InvalidRevision(u32, u32),

    /// Unknown header revision (from the future)
    #[error("unknown header revision {0}")]
    UnknownRevision(u32),

    /// Invalid header, could not validate magic
    #[error("invalid header")]
    InvalidHeader,

    /// Invalid service identifeir (sid)
    #[error("invalid sid")]
    InvalidSid,

    /// Unexpected service identifier (sid)
    #[error("unexpected sid, expected {} but got {}",
        .expected.map_or_else(|| "none".to_string(), |n| n.to_string()),
        .got.map_or_else(|| "none".to_string(), |n| n.to_string()))]
    UnexpectedSid {
        expected: Option<u32>,
        got: Option<u32>,
    },

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
        let (revision, plain_secret) = PlainSecret::create_latest(key.into(), iv.into(), settings)?;

        Ok(Header {
            revision,
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
            Revision::Rev2(data) => Self::read_rev2(data, migrator, store),
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

    fn read_rev2(
        data: Data,
        migrator: Migrator<'a>,
        store: &mut PasswordStore,
    ) -> Result<Header<'a, B>, HeaderError> {
        let key = Self::create_key(data.cipher, &data.kdf, store)?;
        let mut ctx = Self::prepare_cipher_ctx(data.cipher, &data.secret);

        let pbuf = ctx.decrypt(&key, &data.iv)?;
        let plain_secret = PlainSecret::from_buffer_rev2(&mut &pbuf[..])?;

        Ok(Header {
            revision: 2,
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
            PlainSecret::Rev2(_) => Revision::new_rev2(self.cipher, iv, self.kdf.clone(), secret),
        };

        rev.put_into_buffer(&mut &mut buf[..])
    }

    pub fn migrate(&mut self) -> Result<(), HeaderError> {
        if let PlainSecret::Rev0(rev0) = &mut self.data {
            rev0.migrate(&self.migrator)
        } else {
            Ok(())
        }
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn latest_revision_or_err(&self) -> Result<(), HeaderError> {
        if self.revision == LATEST_REVISION {
            Ok(())
        } else {
            Err(HeaderError::InvalidRevision(LATEST_REVISION, self.revision))
        }
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
            PlainSecret::Rev2(rev2) => &rev2.settings,
        }
    }

    pub fn key(&self) -> &[u8] {
        match &self.data {
            PlainSecret::Rev0(rev0) => &rev0.key,
            PlainSecret::Rev1(rev1) => &rev1.key,
            PlainSecret::Rev2(rev2) => &rev2.key,
        }
    }

    pub fn iv(&self) -> &[u8] {
        match &self.data {
            PlainSecret::Rev0(rev0) => &rev0.iv,
            PlainSecret::Rev1(rev1) => &rev1.iv,
            PlainSecret::Rev2(rev2) => &rev2.iv,
        }
    }

    pub fn accept_sid_for_create(&self) -> Result<(), HeaderError> {
        let sid_opt = match &self.data {
            PlainSecret::Rev0(rev0) => rev0.sid,
            PlainSecret::Rev1(_) => None,
            PlainSecret::Rev2(rev2) => rev2.sid,
        };

        if sid_opt.is_none() {
            Ok(())
        } else {
            Err(HeaderError::UnexpectedSid {
                expected: None,
                got: sid_opt,
            })
        }
    }

    pub fn accept_sid_for_open(&self, sid: u32) -> Result<(), HeaderError> {
        let accecpt = |header_sid| match header_sid {
            Some(hsid) if hsid == sid => {
                debug!("sid {} match", sid);

                Ok(())
            }
            _ => {
                error!("sid mismatch, sid: {}, header sid: {:?}", sid, header_sid);

                Err(HeaderError::UnexpectedSid {
                    expected: Some(sid),
                    got: header_sid,
                })
            }
        };

        match &self.data {
            PlainSecret::Rev0(rev0) => accecpt(rev0.sid),
            PlainSecret::Rev1(_) => {
                // There is no way to identify a rev 1 service. This is the reason we
                // put the sid (service identifier) into the header.
                // Assume that the service rejects invalid data from its super-block.
                // It's ok to say ok here.

                debug!("rev1 has no sid, say ok");

                Ok(())
            }
            PlainSecret::Rev2(rev2) => accecpt(rev2.sid),
        }
    }

    pub fn set_sid(&mut self, sid: u32) -> Result<(), HeaderError> {
        match &mut self.data {
            PlainSecret::Rev0(_) => panic!("storing a sid into a rev0 header is not supported"),
            PlainSecret::Rev1(_) => panic!("storing a sid into a rev1 header is not supported"),
            PlainSecret::Rev2(rev2) => {
                if sid > 0 {
                    rev2.sid = Some(sid);
                    Ok(())
                } else {
                    Err(HeaderError::InvalidSid)
                }
            }
        }
    }

    pub fn top_id(&self) -> Option<&B::Id> {
        match &self.data {
            PlainSecret::Rev0(rev0) => rev0.top_id.as_ref(),
            PlainSecret::Rev1(rev1) => rev1.top_id.as_ref(),
            PlainSecret::Rev2(rev2) => rev2.top_id.as_ref(),
        }
    }

    pub fn set_top_id(&mut self, id: B::Id) {
        match &mut self.data {
            PlainSecret::Rev0(_) => panic!("storing a top-id into a rev0 header is not supported"),
            PlainSecret::Rev1(_) => panic!("storing a top-id into a rev1 header is not supported"),
            PlainSecret::Rev2(rev2) => rev2.top_id = Some(id),
        }
    }

    pub fn set_migrator(&mut self, migrator: Migrator<'a>) {
        self.migrator = migrator;
    }

    pub fn convert_to_latest(&mut self, sid: u32) -> bool {
        self.data.convert_to_latest(sid)
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
