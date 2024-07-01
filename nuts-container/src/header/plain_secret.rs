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

use crate::buffer::{Buffer, BufferError, BufferMut, FromBuffer, ToBuffer};
use crate::cipher::{Cipher, CipherContext};
use crate::header::secret::Secret;
use crate::header::HeaderError;
use crate::kdf::Kdf;
use crate::ossl;
use crate::password::PasswordStore;
use crate::svec::SecureVec;

fn generate_magics() -> Result<[u32; 2], ErrorStack> {
    ossl::rand_u32().map(|magic| [magic, magic])
}

pub trait Encryptor: ToBuffer + Sized {
    fn encrypt(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> Result<Secret, HeaderError> {
        let mut pbuf: SecureVec = vec![].into();
        self.to_buffer(pbuf.deref_mut())?;

        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut ctx = CipherContext::new(cipher);

        ctx.copy_from_slice(pbuf.len(), &pbuf);
        let cbuf = ctx.encrypt(&key, iv)?;

        Ok(Secret::new(cbuf.to_vec()))
    }
}

#[derive(Debug, PartialEq)]
pub struct PlainSecretRev0 {
    magics: [u32; 2],
    pub key: SecureVec,
    pub iv: SecureVec,
    pub userdata: SecureVec,
    pub settings: SecureVec,
}

impl PlainSecretRev0 {
    pub fn generate(
        key: SecureVec,
        iv: SecureVec,
        userdata: SecureVec,
        settings: SecureVec,
    ) -> Result<PlainSecretRev0, ErrorStack> {
        Ok(PlainSecretRev0 {
            magics: generate_magics()?,
            key,
            iv,
            userdata,
            settings,
        })
    }
}

impl FromBuffer for PlainSecretRev0 {
    type Error = HeaderError;

    fn from_buffer<T: Buffer>(buf: &mut T) -> Result<Self, HeaderError> {
        let magic1 = buf.get_u32()?;
        let magic2 = buf.get_u32()?;

        if magic1 != magic2 {
            return Err(HeaderError::WrongPassword);
        }

        let key = buf.get_vec::<8>()?.into();
        let iv = buf.get_vec::<8>()?.into();
        let userdata = buf.get_vec::<8>()?.into();
        let settings = buf.get_vec::<8>()?.into();

        Ok(PlainSecretRev0 {
            magics: [magic1, magic2],
            key,
            iv,
            userdata,
            settings,
        })
    }
}

impl ToBuffer for PlainSecretRev0 {
    fn to_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        buf.put_u32(self.magics[0])?;
        buf.put_u32(self.magics[1])?;
        buf.put_vec::<8>(&self.key)?;
        buf.put_vec::<8>(&self.iv)?;
        buf.put_vec::<8>(&self.userdata)?;
        buf.put_vec::<8>(&self.settings)?;

        Ok(())
    }
}

impl Encryptor for PlainSecretRev0 {}
