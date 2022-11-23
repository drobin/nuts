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

#[cfg(test)]
mod tests;

use std::io::{Read, Write};
use std::marker::PhantomData;

use crate::backend::Backend;
use crate::bytes::{self, FromBytes, FromBytesExt, ToBytes, ToBytesExt};
use crate::container::error::ContainerResult;
use crate::openssl::evp;
use crate::whiteout_vec;

/// Supported cipher algorithms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cipher {
    /// No encryption.
    None,

    /// AES with a 128-bit key in CTR mode
    Aes128Ctr,
}

impl Cipher {
    /// Returns the block size of the cipher.
    pub fn block_size(&self) -> usize {
        match self.to_evp() {
            None => 1,
            Some(c) => c.block_size(),
        }
    }

    /// Returns the key size of the cipher.
    pub fn key_len(&self) -> usize {
        match self.to_evp() {
            None => 0,
            Some(c) => c.key_length(),
        }
    }

    /// Returns the IV size of the cipher.
    pub fn iv_len(&self) -> usize {
        match self.to_evp() {
            None => 0,
            Some(c) => c.iv_length(),
        }
    }

    fn to_evp(&self) -> Option<evp::Cipher> {
        match self {
            Cipher::None => None,
            Cipher::Aes128Ctr => Some(evp::Cipher::aes128_ctr()),
        }
    }
}

impl FromBytes for Cipher {
    fn from_bytes<R: Read>(source: &mut R) -> bytes::Result<Self> {
        let n = source.from_bytes()?;

        match n {
            0u8 => Ok(Cipher::None),
            1u8 => Ok(Cipher::Aes128Ctr),
            _ => Err(bytes::Error::invalid(format!("invalid cipher: {}", n))),
        }
    }
}

impl ToBytes for Cipher {
    fn to_bytes<W: Write>(&self, target: &mut W) -> bytes::Result<()> {
        let n = match self {
            Cipher::None => 0u8,
            Cipher::Aes128Ctr => 1u8,
        };

        target.to_bytes(&n)
    }
}

pub struct CipherCtx<B> {
    _data: PhantomData<B>,
    ctx: evp::CipherCtx,
    cipher: Cipher,
    out: Vec<u8>,
}

impl<B: Backend> CipherCtx<B> {
    pub fn new(cipher: Cipher) -> ContainerResult<CipherCtx<B>, B> {
        Ok(CipherCtx {
            _data: PhantomData,
            ctx: evp::CipherCtx::new()?,
            cipher,
            out: vec![],
        })
    }

    pub fn encrypt(&mut self, key: &[u8], iv: &[u8], input: &[u8]) -> ContainerResult<&[u8], B> {
        match self.cipher.to_evp() {
            Some(cipher) => self.encrypt_some(cipher, key, iv, input),
            None => self.update_none(input),
        }
    }

    pub fn decrypt(&mut self, key: &[u8], iv: &[u8], input: &[u8]) -> ContainerResult<&[u8], B> {
        match self.cipher.to_evp() {
            Some(cipher) => self.decrypt_some(cipher, key, iv, input),
            None => self.update_none(input),
        }
    }

    fn encrypt_some(
        &mut self,
        cipher: evp::Cipher,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        self.out.resize(input.len(), 0);
        let n = self.ctx.encrypt(cipher, key, iv, input, &mut self.out)?;

        self.out.resize(n, 0);

        Ok(&self.out)
    }

    fn decrypt_some(
        &mut self,
        cipher: evp::Cipher,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        self.out.resize(input.len(), 0);
        let n = self.ctx.decrypt(cipher, key, iv, input, &mut self.out)?;

        self.out.resize(n, 0);

        Ok(&self.out)
    }

    fn update_none(&mut self, input: &[u8]) -> ContainerResult<&[u8], B> {
        self.out.clear();
        self.out.extend_from_slice(input);

        Ok(&self.out)
    }
}

impl<B> Drop for CipherCtx<B> {
    fn drop(&mut self) {
        whiteout_vec(&mut self.out);
    }
}
