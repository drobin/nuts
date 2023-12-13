// MIT License
//
// Copyright (c) 2023 Robin Doer
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

#[cfg(test)]
mod tests;

use nuts_bytes::{FromBytes, Reader, ToBytes, Writer};
use openssl::error::ErrorStack;

use crate::container::cipher::{Cipher, CipherContext};
use crate::container::header::{HeaderError, SecretMagicsError};
use crate::container::kdf::Kdf;
use crate::container::ossl;
use crate::container::password::PasswordStore;
use crate::container::svec::SecureVec;

fn generate_magics() -> Result<[u32; 2], ErrorStack> {
    ossl::rand_u32().map(|magic| [magic, magic])
}

fn validate_magics(magics: [u32; 2]) -> Result<[u32; 2], SecretMagicsError> {
    if magics[0] == magics[1] {
        Ok(magics)
    } else {
        Err(SecretMagicsError)
    }
}

#[derive(Debug, FromBytes, PartialEq, ToBytes)]
pub struct Secret(Vec<u8>);

impl Secret {
    #[cfg(test)]
    pub fn new(vec: Vec<u8>) -> Secret {
        Secret(vec)
    }

    pub fn decrypt(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> Result<PlainSecret, HeaderError> {
        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut ctx = CipherContext::new(cipher);

        ctx.copy_from_slice(self.0.len(), &self.0);

        let pbuf = ctx.decrypt(&key, &iv)?;
        let plain_secret = Reader::new(pbuf).read()?;

        Ok(plain_secret)
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for Secret {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

#[derive(Debug, FromBytes, PartialEq, ToBytes)]
pub struct PlainSecret {
    #[nuts_bytes(map_from_bytes = validate_magics)]
    magics: [u32; 2],
    pub key: SecureVec,
    pub iv: SecureVec,
    pub userdata: SecureVec,
    pub settings: SecureVec,
}

impl PlainSecret {
    pub fn generate(
        key: SecureVec,
        iv: SecureVec,
        userdata: SecureVec,
        settings: SecureVec,
    ) -> Result<PlainSecret, ErrorStack> {
        Ok(PlainSecret {
            magics: generate_magics()?,
            key,
            iv,
            userdata,
            settings,
        })
    }

    pub fn encrypt(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> Result<Secret, HeaderError> {
        let mut writer = Writer::new(vec![]);
        let pbuf: SecureVec = writer.write(&self).map(|_| writer.into_target().into())?;

        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut ctx = CipherContext::new(cipher);

        ctx.copy_from_slice(pbuf.len(), &pbuf);
        let cbuf = ctx.encrypt(&key, &iv)?;

        Ok(Secret(cbuf.to_vec()))
    }
}
