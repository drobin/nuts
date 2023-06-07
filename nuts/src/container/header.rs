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

mod inner;
mod rev0;
mod secret;

use std::error;
use std::fmt::{self, Write as FmtWrite};

use nuts_backend::Backend;
use nuts_bytes::{Reader, Writer};
use serde::Serialize;

use crate::container::cipher::Cipher;
use crate::container::header::inner::{Inner, Revision};
use crate::container::kdf::Kdf;
use crate::container::options::CreateOptions;
use crate::container::password::{NoPasswordError, PasswordStore};
use crate::openssl::{rand, OpenSSLError};
use crate::svec::SecureVec;

use self::secret::PlainSecret;

#[derive(Debug)]
pub enum HeaderError {
    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    OpenSSL(OpenSSLError),

    /// A password is needed by the current cipher.
    NoPassword(NoPasswordError),

    /// The password is wrong.
    WrongPassword(nuts_bytes::Error),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Self::NoPassword(cause) => fmt::Display::fmt(cause, fmt),
            Self::WrongPassword(_) => write!(fmt, "The password is wrong."),
        }
    }
}

impl error::Error for HeaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Bytes(cause) => Some(cause),
            Self::OpenSSL(cause) => Some(cause),
            Self::NoPassword(cause) => Some(cause),
            Self::WrongPassword(cause) => Some(cause),
        }
    }
}

impl From<nuts_bytes::Error> for HeaderError {
    fn from(cause: nuts_bytes::Error) -> Self {
        match &cause {
            nuts_bytes::Error::Serde(msg) => {
                if msg == "secret-magic mismatch" {
                    return HeaderError::WrongPassword(cause);
                }
            }
            _ => {}
        }

        HeaderError::Bytes(cause)
    }
}

impl From<OpenSSLError> for HeaderError {
    fn from(cause: OpenSSLError) -> Self {
        HeaderError::OpenSSL(cause)
    }
}

impl From<NoPasswordError> for HeaderError {
    fn from(cause: NoPasswordError) -> Self {
        HeaderError::NoPassword(cause)
    }
}

pub struct Header<B: Backend> {
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Kdf,
    pub(crate) key: SecureVec,
    pub(crate) iv: SecureVec,
    pub(crate) top_id: Option<B::Id>,
}

impl<B: Backend> Header<B> {
    pub fn create(options: &CreateOptions<B>) -> Result<Header<B>, HeaderError> {
        let cipher = options.cipher;
        let mut key = vec![0; cipher.key_len()];
        let mut iv = vec![0; cipher.iv_len()];

        rand::rand_bytes(&mut key)?;
        rand::rand_bytes(&mut iv)?;

        let kdf = options.kdf.build()?;

        Ok(Header {
            cipher,
            kdf,
            key: key.into(),
            iv: iv.into(),
            top_id: None,
        })
    }

    pub fn read(
        buf: &[u8],
        store: &mut PasswordStore,
    ) -> Result<(Header<B>, B::Settings), HeaderError> {
        let inner = Reader::new(buf).deserialize::<Inner>()?;

        let Revision::Rev0(rev0) = inner.rev;

        let plain_secret = rev0
            .secret
            .decrypt::<B>(store, rev0.cipher, &rev0.kdf, &rev0.iv)?;

        Ok((
            Header {
                cipher: rev0.cipher,
                kdf: rev0.kdf,
                key: plain_secret.key,
                iv: plain_secret.iv,
                top_id: plain_secret.top_id,
            },
            plain_secret.settings,
        ))
    }

    pub fn write(
        &self,
        settings: B::Settings,
        buf: &mut [u8],
        store: &mut PasswordStore,
    ) -> Result<(), HeaderError> {
        let plain_secret = PlainSecret::<B>::generate(
            self.key.clone(),
            self.iv.clone(),
            self.top_id.clone(),
            settings,
        )?;

        let mut iv = vec![0; self.cipher.iv_len()];
        rand::rand_bytes(&mut iv)?;

        let secret = plain_secret.encrypt(store, self.cipher, &self.kdf, &iv)?;

        let rev0 = rev0::Data {
            cipher: self.cipher,
            iv,
            kdf: self.kdf.clone(),
            secret,
        };
        let inner = Inner::new(Revision::Rev0(rev0));

        let mut writer = Writer::new(buf);
        inner.serialize(&mut writer)?;

        Ok(())
    }
}

impl<B: Backend> fmt::Debug for Header<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (key, iv) = if cfg!(feature = "debug-plain-keys") && cfg!(debug_assertions) {
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
