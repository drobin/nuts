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

mod inner;
mod rev0;
mod secret;

use nuts_backend::Backend;
use nuts_bytes::{Reader, Writer};
use openssl::error::ErrorStack;
use std::fmt::{self, Write as FmtWrite};
use thiserror::Error;

use crate::cipher::{Cipher, CipherError};
use crate::header::inner::{Inner, Revision};
use crate::header::secret::PlainSecret;
use crate::kdf::{Kdf, KdfError};
use crate::options::CreateOptions;
use crate::ossl;
use crate::password::{PasswordError, PasswordStore};
use crate::svec::SecureVec;

/// The magic that marks the header is wrong.
#[derive(Debug, Error)]
#[error("invalid header")]
struct HeaderMagicError;

/// The magic number pair in the secret is wrong.
#[derive(Debug, Error)]
#[error("")]
struct SecretMagicsError;

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
    WrongPassword(#[source] nuts_bytes::Error),

    /// Error while (de-) serializing binary data.
    #[error(transparent)]
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    #[error(transparent)]
    OpenSSL(#[from] ErrorStack),
}

impl From<nuts_bytes::Error> for HeaderError {
    fn from(value: nuts_bytes::Error) -> Self {
        match &value {
            nuts_bytes::Error::Custom(err) => {
                if err.is::<SecretMagicsError>() {
                    return Self::WrongPassword(value);
                }
            }
            _ => {}
        };

        Self::Bytes(value)
    }
}

pub struct Header {
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Kdf,
    pub(crate) key: SecureVec,
    pub(crate) iv: SecureVec,
    pub(crate) userdata: SecureVec,
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
            userdata: vec![].into(),
        })
    }

    pub fn read<B: Backend>(
        buf: &[u8],
        store: &mut PasswordStore,
    ) -> Result<(Header, B::Settings), HeaderError> {
        let inner = Reader::new(buf).read::<Inner>()?;

        let Revision::Rev0(rev0) = inner.rev;

        let plain_secret = rev0
            .secret
            .decrypt(store, rev0.cipher, &rev0.kdf, &rev0.iv)?;

        let settings = Reader::new(plain_secret.settings.as_slice()).read()?;

        Ok((
            Header {
                cipher: rev0.cipher,
                kdf: rev0.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                userdata: plain_secret.userdata,
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
        let mut writer = Writer::new(vec![]);
        writer.write(&settings)?;

        let plain_secret = PlainSecret::generate(
            self.key.clone(),
            self.iv.clone(),
            self.userdata.clone(),
            writer.into_target().into(),
        )?;

        let mut iv = vec![0; self.cipher.iv_len()];
        ossl::rand_bytes(&mut iv)?;

        let secret = plain_secret.encrypt(store, self.cipher, &self.kdf, &iv)?;

        let rev0 = rev0::Data {
            cipher: self.cipher,
            iv,
            kdf: self.kdf.clone(),
            secret,
        };
        let inner = Inner::new(Revision::Rev0(rev0));

        Writer::new(buf).write(&inner)?;

        Ok(())
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
