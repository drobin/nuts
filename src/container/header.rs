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

use nuts_bytes::{Reader, Writer};
use openssl::error::ErrorStack;
use std::error;
use std::fmt::{self, Write as FmtWrite};

use crate::backend::Backend;
use crate::container::cipher::Cipher;
use crate::container::cipher::CipherError;
use crate::container::header::inner::{Inner, Revision};
use crate::container::header::secret::PlainSecret;
use crate::container::kdf::Kdf;
use crate::container::options::CreateOptions;
use crate::container::ossl;
use crate::container::password::{PasswordError, PasswordStore};
use crate::container::svec::SecureVec;

#[derive(Debug)]
pub enum HeaderError {
    /// Error while (de-) serializing binary data.
    Bytes(nuts_bytes::Error),

    /// An error in the OpenSSL library occured.
    OpenSSL(ErrorStack),

    /// An error occured in a cipher operation.
    Cipher(CipherError),

    /// A password is needed by the current cipher.
    Password(PasswordError),

    /// The password is wrong.
    WrongPassword(nuts_bytes::Error),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(cause) => fmt::Display::fmt(cause, fmt),
            Self::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Self::Cipher(cause) => fmt::Display::fmt(cause, fmt),
            Self::Password(cause) => fmt::Display::fmt(cause, fmt),
            Self::WrongPassword(_) => write!(fmt, "The password is wrong."),
        }
    }
}

impl error::Error for HeaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Bytes(cause) => Some(cause),
            Self::OpenSSL(cause) => Some(cause),
            Self::Cipher(cause) => Some(cause),
            Self::Password(cause) => Some(cause),
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

impl From<ErrorStack> for HeaderError {
    fn from(cause: ErrorStack) -> Self {
        HeaderError::OpenSSL(cause)
    }
}

impl From<CipherError> for HeaderError {
    fn from(cause: CipherError) -> Self {
        HeaderError::Cipher(cause)
    }
}

impl From<PasswordError> for HeaderError {
    fn from(cause: PasswordError) -> Self {
        HeaderError::Password(cause)
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
                userdata: plain_secret.userdata,
            },
            plain_secret.settings,
        ))
    }

    pub fn write<B: Backend>(
        &self,
        settings: B::Settings,
        buf: &mut [u8],
        store: &mut PasswordStore,
    ) -> Result<(), HeaderError> {
        let plain_secret = PlainSecret::<B>::generate(
            self.key.clone(),
            self.iv.clone(),
            self.userdata.clone(),
            settings,
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

        Writer::new(buf).serialize(&inner)?;

        Ok(())
    }
}

impl fmt::Debug for Header {
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
