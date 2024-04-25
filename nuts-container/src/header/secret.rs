// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use openssl::error::ErrorStack;
use std::ops::DerefMut;

use crate::buffer::{Buffer, BufferMut};
use crate::cipher::{Cipher, CipherContext};
use crate::header::HeaderError;
use crate::kdf::Kdf;
use crate::ossl;
use crate::password::PasswordStore;
use crate::svec::SecureVec;

fn generate_magics() -> Result<[u32; 2], ErrorStack> {
    ossl::rand_u32().map(|magic| [magic, magic])
}

#[derive(Debug, PartialEq)]
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
        let plain_secret = PlainSecret::get_from_buffer(&mut &pbuf[..])?;

        Ok(plain_secret)
    }

    pub fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Secret, HeaderError> {
        let vec = buf.get_vec()?;

        Ok(Secret(vec))
    }

    pub fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        buf.put_vec(&self.0).map_err(|err| err.into())
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for Secret {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct PlainSecret {
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
        let mut pbuf: SecureVec = vec![].into();
        self.put_into_buffer(pbuf.deref_mut())?;

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

    pub fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<PlainSecret, HeaderError> {
        let magic1 = buf.get_u32()?;
        let magic2 = buf.get_u32()?;

        if magic1 != magic2 {
            return Err(HeaderError::WrongPassword);
        }

        let key = buf.get_vec()?.into();
        let iv = buf.get_vec()?.into();
        let userdata = buf.get_vec()?.into();
        let settings = buf.get_vec()?.into();

        Ok(PlainSecret {
            magics: [magic1, magic2],
            key,
            iv,
            userdata,
            settings,
        })
    }

    pub fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        buf.put_u32(self.magics[0])?;
        buf.put_u32(self.magics[1])?;
        buf.put_vec(&self.key)?;
        buf.put_vec(&self.iv)?;
        buf.put_vec(&self.userdata)?;
        buf.put_vec(&self.settings)?;

        Ok(())
    }
}
