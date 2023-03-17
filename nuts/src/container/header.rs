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
mod settings;

use std::fmt::{self, Write as FmtWrite};

use nuts_backend::Backend;

use crate::bytes::Options;
use crate::container::cipher::Cipher;
use crate::container::error::ContainerResult;
use crate::container::header::inner::{Inner, Revision};
use crate::container::kdf::Kdf;
use crate::container::options::CreateOptions;
use crate::container::password::PasswordStore;
use crate::openssl::rand;
use crate::svec::SecureVec;

use self::secret::PlainSecret;
use self::settings::Settings;

fn bytes_options() -> Options {
    Options::new().with_varint()
}

pub struct Header {
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Kdf,
    pub(crate) key: SecureVec,
    pub(crate) iv: SecureVec,
}

impl Header {
    pub fn create<B: Backend>(options: &CreateOptions<B>) -> ContainerResult<Header, B> {
        let cipher = options.cipher;
        let mut key = SecureVec::zero(cipher.key_len());
        let mut iv = SecureVec::zero(cipher.iv_len());

        rand::rand_bytes(&mut key)?;
        rand::rand_bytes(&mut iv)?;

        let kdf = options.kdf.build()?;

        Ok(Header {
            cipher,
            kdf,
            key,
            iv,
        })
    }

    pub fn read<B: Backend>(
        buf: &[u8],
        store: &mut PasswordStore,
    ) -> ContainerResult<(Header, B::Settings), B> {
        let inner = bytes_options().from_bytes::<Inner>(buf)?;
        let Revision::Rev0(rev0) = inner.rev;

        let plain_secret = rev0
            .secret
            .decrypt(store, rev0.cipher, &rev0.kdf, &rev0.iv)?;

        Ok((
            Header {
                cipher: rev0.cipher,
                kdf: rev0.kdf,
                key: plain_secret.key.clone(),
                iv: plain_secret.iv.clone(),
            },
            plain_secret.settings.into_backend()?,
        ))
    }

    pub fn write<B: Backend>(
        &self,
        settings: &B::Settings,
        buf: &mut [u8],
        store: &mut PasswordStore,
    ) -> ContainerResult<(), B> {
        let settings = Settings::from_backend(settings)?;
        let plain_secret = PlainSecret::generate(self.key.clone(), self.iv.clone(), settings)?;

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

        bytes_options().to_bytes(&inner, buf)?;

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
