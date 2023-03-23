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

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use nuts_backend::Backend;

use crate::container::error::ContainerResult;
use crate::openssl::evp;
use crate::svec::SecureVec;

/// Supported cipher algorithms.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

pub struct CipherCtx {
    ctx: evp::CipherCtx,
    cipher: Cipher,
    block_size: usize,
    out: SecureVec,
}

impl CipherCtx {
    pub fn new<B: Backend>(cipher: Cipher, block_size: u32) -> ContainerResult<CipherCtx, B> {
        Ok(CipherCtx {
            ctx: evp::CipherCtx::new()?,
            cipher,
            block_size: block_size as usize,
            out: vec![].into(),
        })
    }

    pub fn encrypt<B: Backend>(
        &mut self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        let ptext = self.prepare_ptext(input);

        match self.cipher.to_evp() {
            Some(cipher) => self.encrypt_some(cipher, key, iv, &ptext),
            None => self.update_none(&ptext),
        }
    }

    pub fn decrypt<B: Backend>(
        &mut self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        match self.cipher.to_evp() {
            Some(cipher) => self.decrypt_some(cipher, key, iv, input),
            None => self.update_none(input),
        }
    }

    fn prepare_ptext<'a>(&self, input: &'a [u8]) -> Cow<'a, [u8]> {
        let mut ptext = Cow::from(input);

        if ptext.len() < self.block_size {
            // pad with 0 if not a complete block
            ptext.to_mut().resize(self.block_size, 0);
        }

        ptext
    }

    fn encrypt_some<B: Backend>(
        &mut self,
        cipher: evp::Cipher,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        self.out.resize(input.len(), 0);

        self.ctx.reset()?;
        self.ctx.init(cipher, evp::CipherMode::Encrypt, key, iv)?;
        let n = self.ctx.update(input, &mut self.out)?;

        self.out.resize(n, 0);

        Ok(&self.out)
    }

    fn decrypt_some<B: Backend>(
        &mut self,
        cipher: evp::Cipher,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> ContainerResult<&[u8], B> {
        self.out.resize(input.len(), 0);

        self.ctx.reset()?;
        self.ctx.init(cipher, evp::CipherMode::Decrypt, key, iv)?;
        let n = self.ctx.update(input, &mut self.out)?;

        self.out.resize(n, 0);

        Ok(&self.out)
    }

    fn update_none<B: Backend>(&mut self, input: &[u8]) -> ContainerResult<&[u8], B> {
        self.out.clear();
        self.out.extend_from_slice(input);

        Ok(&self.out)
    }
}
