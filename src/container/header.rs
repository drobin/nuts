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
use crate::container::cipher::Cipher;
use crate::container::error::ContainerResult;
use crate::container::options::CreateOptions;
use crate::openssl::rand;
use crate::whiteout_vec;

const MAGIC: [u8; 7] = *b"nuts-io";

struct Secret<'a> {
    key: Cow<'a, [u8]>,
    iv: Cow<'a, [u8]>,
}

impl<'a> Secret<'a> {
    fn owned(key: Vec<u8>, iv: Vec<u8>) -> Secret<'a> {
        Secret {
            key: Cow::Owned(key),
            iv: Cow::Owned(iv),
        }
    }

    fn borrowed(key: &'a [u8], iv: &'a [u8]) -> Secret<'a> {
        Secret {
            key: Cow::Borrowed(key),
            iv: Cow::Borrowed(iv),
        }
    }
}

impl<'a> FromBytes for Secret<'a> {
    fn from_bytes<R: Read>(source: &mut R) -> bytes::Result<Self> {
        let key = source.from_bytes()?;
        let iv = source.from_bytes()?;

        Ok(Secret::owned(key, iv))
    }
}

impl<'a> ToBytes for Secret<'a> {
    fn to_bytes<W: Write>(&self, target: &mut W) -> bytes::Result<()> {
        target.to_bytes(&&*self.key)?;
        target.to_bytes(&&*self.iv)?;

        Ok(())
    }
}

pub struct Header {
    pub(crate) cipher: Cipher,
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

        Ok(Header { cipher, key, iv })
    }

    pub fn read<B: Backend>(buf: &[u8]) -> bytes::Result<(Header, B::Settings)> {
        let mut cursor = Cursor::new(buf);
        let mut magic = [0; 7];

        cursor.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(bytes::Error::invalid("magic mismatch"));
        }

        let cipher = cursor.from_bytes()?;
        let secret = cursor.from_bytes::<Secret>()?;
        let settings = cursor.from_bytes()?;

        Ok((
            Header {
                cipher,
                key: secret.key.into_owned(),
                iv: secret.iv.into_owned(),
            },
            settings,
        ))
    }

    pub fn write<B: Backend>(&self, settings: &B::Settings, buf: &mut [u8]) -> bytes::Result<()> {
        let secret = Secret::borrowed(&self.key, &self.iv);
        let mut cursor = Cursor::new(buf);

        cursor.write_all(&MAGIC).unwrap();
        cursor.to_bytes(&self.cipher)?;
        cursor.to_bytes(&secret)?;
        cursor.to_bytes(settings)?;
        cursor.flush()?;

        Ok(())
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
