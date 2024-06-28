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

use crate::buffer::{Buffer, BufferError, BufferMut, FromBuffer, ToBuffer};
use crate::cipher::{Cipher, CipherContext};
use crate::header::HeaderError;
use crate::kdf::Kdf;
use crate::password::PasswordStore;

#[derive(Debug, PartialEq)]
pub struct Secret(Vec<u8>);

impl Secret {
    pub fn new(vec: Vec<u8>) -> Secret {
        Secret(vec)
    }

    pub fn decrypt<T: FromBuffer<Error = HeaderError>>(
        self,
        store: &mut PasswordStore,
        cipher: Cipher,
        kdf: &Kdf,
        iv: &[u8],
    ) -> Result<T, HeaderError> {
        let key = if cipher.key_len() > 0 {
            let password = store.value()?;
            kdf.create_key(password)?
        } else {
            vec![].into()
        };

        let mut ctx = CipherContext::new(cipher);

        ctx.copy_from_slice(self.as_ref().len(), self.as_ref());

        let pbuf = ctx.decrypt(&key, iv)?;
        let plain_secret = T::from_buffer(&mut &pbuf[..])?;

        Ok(plain_secret)
    }
}

impl AsRef<[u8]> for Secret {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const L: usize> PartialEq<[u8; L]> for Secret {
    fn eq(&self, other: &[u8; L]) -> bool {
        self.0 == other
    }
}

impl FromBuffer for Secret {
    type Error = BufferError;

    fn from_buffer<T: Buffer>(buf: &mut T) -> Result<Self, BufferError> {
        let vec = buf.get_vec::<8>()?;

        Ok(Secret(vec))
    }
}

impl ToBuffer for Secret {
    fn to_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        buf.put_vec::<8>(&self.0)
    }
}
