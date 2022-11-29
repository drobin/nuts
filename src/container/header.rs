// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use std::borrow::Cow;
use std::fmt::{self, Write as FmtWrite};
use std::io::{Cursor, Read, Write};

use crate::backend::Backend;
use crate::bytes::{self, FromBytes, FromBytesExt, ToBytes, ToBytesExt};
use crate::container::cipher::{Cipher, CipherCtx};
use crate::container::digest::Digest;
use crate::container::error::ContainerResult;
use crate::container::kdf::Kdf;
use crate::container::options::CreateOptions;
use crate::openssl::rand;
use crate::whiteout_vec;

const MAGIC: [u8; 7] = *b"nuts-io";

struct Secret<'a, B: Backend> {
    key: Cow<'a, [u8]>,
    iv: Cow<'a, [u8]>,
    settings: Cow<'a, B::Settings>,
}

impl<'a, B: Backend> Secret<'a, B> {
    fn owned(key: Vec<u8>, iv: Vec<u8>, settings: B::Settings) -> Secret<'a, B> {
        Secret {
            key: Cow::Owned(key),
            iv: Cow::Owned(iv),
            settings: Cow::Owned(settings),
        }
    }

    fn borrowed(key: &'a [u8], iv: &'a [u8], settings: &'a B::Settings) -> Secret<'a, B> {
        Secret {
            key: Cow::Borrowed(key),
            iv: Cow::Borrowed(iv),
            settings: Cow::Borrowed(settings),
        }
    }
}

impl<'a, B: Backend> FromBytes for Secret<'a, B> {
    fn from_bytes<R: Read>(source: &mut R) -> bytes::Result<Self> {
        let key = source.from_bytes()?;
        let iv = source.from_bytes()?;
        let settings = source.from_bytes()?;

        Ok(Secret::owned(key, iv, settings))
    }
}

impl<'a, B: Backend> ToBytes for Secret<'a, B> {
    fn to_bytes<W: Write>(&self, target: &mut W) -> bytes::Result<()> {
        target.to_bytes(&&*self.key)?;
        target.to_bytes(&&*self.iv)?;
        target.to_bytes(self.settings.as_ref())?;

        Ok(())
    }
}

pub struct Header {
    pub(crate) cipher: Cipher,
    pub(crate) kdf: Option<Kdf>,
    pub(crate) key: Vec<u8>,
    pub(crate) iv: Vec<u8>,
}

impl Header {
    pub fn create<B: Backend>(options: &CreateOptions<B>) -> ContainerResult<Header, B> {
        let cipher = options.cipher;
        let mut key = vec![0; cipher.key_len()];
        let mut iv = vec![0; cipher.iv_len()];

        rand::rand_bytes(&mut key)?;
        rand::rand_bytes(&mut iv)?;

        let kdf = match cipher {
            Cipher::None => None,
            _ => Some(Kdf::generate_pbkdf2(Digest::Sha1, 65536, 16)?),
        };

        Ok(Header {
            cipher,
            kdf,
            key,
            iv,
        })
    }

    pub fn read<B: Backend>(buf: &[u8]) -> ContainerResult<(Header, B::Settings), B> {
        let mut cursor = Cursor::new(buf);
        let mut magic = [0; 7];

        cursor.read_bytes(&mut magic)?;

        if magic != MAGIC {
            return Err(bytes::Error::invalid("magic mismatch"))?;
        }

        let revision = cursor.from_bytes::<u8>()?;

        if revision != 1 {
            return Err(bytes::Error::invalid(format!(
                "invalid revision: {}",
                revision
            )))?;
        }

        let cipher = cursor.from_bytes()?;

        if cipher == Cipher::None {
            let settings = cursor.from_bytes()?;

            Ok((
                Header {
                    cipher: Cipher::None,
                    kdf: None,
                    key: vec![],
                    iv: vec![],
                },
                settings,
            ))
        } else {
            let iv = cursor.from_bytes()?;
            let kdf = cursor.from_bytes()?;
            let secret = Self::read_secret(cipher, iv, &kdf, cursor)?;

            Ok((
                Header {
                    cipher,
                    kdf: Some(kdf),
                    key: secret.key.into_owned(),
                    iv: secret.iv.into_owned(),
                },
                secret.settings.into_owned(),
            ))
        }
    }

    fn read_secret<'a, B: Backend>(
        cipher: Cipher,
        iv: Vec<u8>,
        kdf: &Kdf,
        mut cursor: Cursor<&[u8]>,
    ) -> ContainerResult<Secret<'a, B>, B> {
        let mut key = kdf.create_key(b"123")?;

        let result = {
            let cbuf = cursor.from_bytes::<Vec<u8>>()?;

            let mut ctx = CipherCtx::new(cipher, cbuf.len() as u32)?;
            let pbuf = ctx.decrypt(&key, &iv, &cbuf)?;

            Ok(Cursor::new(pbuf).from_bytes()?)
        };

        whiteout_vec(&mut key);

        result
    }

    pub fn write<B: Backend>(
        &self,
        settings: &B::Settings,
        buf: &mut [u8],
    ) -> ContainerResult<(), B> {
        let mut cursor = Cursor::new(buf);

        cursor.write_bytes(&MAGIC)?;
        cursor.to_bytes(&1u8)?; // revision
        cursor.to_bytes(&self.cipher)?;

        if self.cipher == Cipher::None {
            cursor.to_bytes(settings)?;

            Ok(())
        } else {
            let secret = Secret::<B>::borrowed(&self.key, &self.iv, settings);
            let mut iv = vec![0; self.cipher.iv_len()];

            rand::rand_bytes(&mut iv)?;

            cursor.to_bytes(&iv.as_ref())?;
            cursor.to_bytes(self.kdf.as_ref().unwrap())?;
            self.write_secret(secret, iv, cursor)
        }
    }

    fn write_secret<B: Backend>(
        &self,
        secret: Secret<B>,
        iv: Vec<u8>,
        mut cursor: Cursor<&mut [u8]>,
    ) -> ContainerResult<(), B> {
        let mut pbuf: Vec<u8> = vec![];
        let mut key = self.kdf.as_ref().unwrap().create_key(b"123")?;

        let result = {
            let mut sec_cursor = Cursor::new(&mut pbuf);

            sec_cursor.to_bytes(&secret)?;

            let mut ctx = CipherCtx::new(self.cipher, pbuf.len() as u32)?;
            let cbuf = ctx.encrypt(&key, &iv, &pbuf)?;

            Ok(cursor.to_bytes(&cbuf)?)
        };

        whiteout_vec(&mut pbuf);
        whiteout_vec(&mut key);

        result
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        whiteout_vec(&mut self.key);
        whiteout_vec(&mut self.iv);
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
            .field("key", &key)
            .field("iv", &iv)
            .finish()
    }
}
