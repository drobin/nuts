// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

use openssl::cipher as ossl_cipher;
use openssl::cipher_ctx::CipherCtx;
use openssl::error::ErrorStack;
use std::str::FromStr;
use std::{cmp, fmt};
use thiserror::Error;

use crate::buffer::{Buffer, BufferError, BufferMut};
use crate::svec::SecureVec;

/// [`Cipher`] related error codes.
#[derive(Debug, Error)]
pub enum CipherError {
    /// The cipher key is invalid/too short.
    #[error("invalid key")]
    InvalidKey,

    /// The cipher iv is invalid/too short.
    #[error("invalid iv")]
    InvalidIv,

    /// The size of the block to be encrypted/decrypted is invalid and must be
    /// aligned at the [block size](Cipher::block_size) of the cipher.
    #[error("invalid block-size")]
    InvalidBlockSize,

    /// A cipher-text is not trustworthy.
    ///
    /// If an authenticated decryption is performed, and the tag mismatches,
    /// this error is raised.
    #[error("the plaintext is not trustworthy")]
    NotTrustworthy,

    /// An error in the OpenSSL library occured.
    #[error(transparent)]
    OpenSSL(#[from] ErrorStack),
}

/// Supported cipher algorithms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cipher {
    /// No encryption.
    None,

    /// AES with a 128-bit key in CTR mode
    Aes128Ctr,

    /// AES with a 128-bit key in GCM mode
    Aes128Gcm,
}

impl Cipher {
    /// Returns the block size of the cipher.
    pub fn block_size(&self) -> usize {
        match self.to_openssl() {
            None => 1,
            Some(c) => c.block_size(),
        }
    }

    /// Returns the key size of the cipher.
    pub fn key_len(&self) -> usize {
        match self.to_openssl() {
            None => 0,
            Some(c) => c.key_length(),
        }
    }

    /// Returns the IV size of the cipher.
    pub fn iv_len(&self) -> usize {
        match self.to_openssl() {
            None => 0,
            Some(c) => c.iv_length(),
        }
    }

    /// Returns the tag size of the cipher.
    ///
    /// An AE-cipher results into a
    ///
    /// 1. ciphertext
    /// 2. tag
    ///
    /// Ciphertext and tag are both stored in a block of the container. Use
    /// this method to get the size of the tag. For a non-AE-cipher the
    /// tag-size is `0`.
    pub fn tag_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 0,
            Cipher::Aes128Gcm => 16,
        }
    }

    pub(crate) fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Cipher, BufferError> {
        let b = buf.get_u32()?;

        match b {
            0 => Ok(Cipher::None),
            1 => Ok(Cipher::Aes128Ctr),
            2 => Ok(Cipher::Aes128Gcm),
            _ => Err(BufferError::InvalidIndex("Cipher".to_string(), b)),
        }
    }

    pub(crate) fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        let b = match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 1,
            Cipher::Aes128Gcm => 2,
        };

        buf.put_u32(b)
    }

    fn to_openssl(self) -> Option<&'static ossl_cipher::CipherRef> {
        match self {
            Cipher::None => None,
            Cipher::Aes128Ctr => Some(ossl_cipher::Cipher::aes_128_ctr()),
            Cipher::Aes128Gcm => Some(ossl_cipher::Cipher::aes_128_gcm()),
        }
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cipher::None => "none",
            Cipher::Aes128Ctr => "aes128-ctr",
            Cipher::Aes128Gcm => "aes128-gcm",
        };

        fmt.write_str(s)
    }
}

impl FromStr for Cipher {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, ()> {
        match str {
            "none" => Ok(Cipher::None),
            "aes128-ctr" => Ok(Cipher::Aes128Ctr),
            "aes128-gcm" => Ok(Cipher::Aes128Gcm),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub(super) struct CipherContext {
    cipher: Cipher,
    inp: SecureVec,
    outp: SecureVec,
}

impl CipherContext {
    pub(super) fn new(cipher: Cipher) -> CipherContext {
        CipherContext {
            cipher,
            inp: vec![].into(),
            outp: vec![].into(),
        }
    }

    pub fn copy_from_slice(&mut self, buf_size: usize, buf: &[u8]) -> usize {
        let len = cmp::min(buf_size, buf.len());

        self.inp.resize(buf_size, 0);
        self.inp[..len].copy_from_slice(&buf[..len]);
        self.inp[len..].iter_mut().for_each(|n| *n = 0);

        len
    }

    pub fn inp_mut(&mut self, buf_size: usize) -> &mut [u8] {
        self.copy_from_slice(buf_size, &[]); // whiteout

        &mut self.inp
    }

    pub fn encrypt(&mut self, key: &[u8], iv: &[u8]) -> Result<&[u8], CipherError> {
        match self.cipher {
            Cipher::None => self.make_none(),
            _ => self.encrypt_aad(None, key, iv).map(|_| ())?,
        };

        Ok(self.outp.as_slice())
    }

    fn encrypt_aad(
        &mut self,
        aad: Option<&[u8]>,
        key: &[u8],
        iv: &[u8],
    ) -> Result<usize, CipherError> {
        let key = key
            .get(..self.cipher.key_len())
            .ok_or(CipherError::InvalidKey)?;
        let iv = iv
            .get(..self.cipher.iv_len())
            .ok_or(CipherError::InvalidIv)?;

        // number of plaintext bytes: equals to number of input bytes. There is
        // no need to align at block size because blocksize is 1 for all
        // ciphers.
        let ptext_len = self.inp.len();

        // number of ciphertext bytes: equals to plaintext bytes (for now) because
        // blocksize is 1 for all ciphers.
        let ctext_len = ptext_len;

        if ptext_len == 0 {
            return Ok(0);
        }

        if ptext_len % self.cipher.block_size() != 0 {
            return Err(CipherError::InvalidBlockSize);
        }

        let mut ctx = CipherCtx::new()?;

        ctx.encrypt_init(self.cipher.to_openssl(), Some(key), Some(iv))?;
        ctx.set_padding(false);

        if let Some(buf) = aad {
            ctx.cipher_update(buf, None)?;
        }

        self.outp
            .resize(ctext_len + self.cipher.tag_size() as usize, 0);
        ctx.cipher_update(&self.inp[..ptext_len], Some(&mut self.outp[..ctext_len]))?;

        if self.cipher.tag_size() > 0 {
            ctx.cipher_final(&mut [])?;
            ctx.tag(&mut self.outp[ctext_len..])?;
        }

        Ok(ctext_len)
    }

    pub fn decrypt(&mut self, key: &[u8], iv: &[u8]) -> Result<&[u8], CipherError> {
        match self.cipher {
            Cipher::None => self.make_none(),
            _ => self.decrypt_aad(None, key, iv).map(|_| ())?,
        }

        Ok(self.outp.as_slice())
    }

    fn decrypt_aad(
        &mut self,
        aad: Option<&[u8]>,
        key: &[u8],
        iv: &[u8],
    ) -> Result<usize, CipherError> {
        let key = key
            .get(..self.cipher.key_len())
            .ok_or(CipherError::InvalidKey)?;
        let iv = iv
            .get(..self.cipher.iv_len())
            .ok_or(CipherError::InvalidIv)?;

        // number of ciphertext bytes: remove tag from the input.
        let ctext_bytes = self
            .inp
            .len()
            .checked_sub(self.cipher.tag_size() as usize)
            .unwrap_or(0);

        // number of plaintext bytes: equals to ciphertext bytes (for now) because
        // blocksize is 1 for all ciphers.
        let ptext_bytes = ctext_bytes;

        if ctext_bytes == 0 {
            return Ok(0);
        }

        if ctext_bytes % self.cipher.block_size() != 0 {
            return Err(CipherError::InvalidBlockSize);
        }

        let mut ctx = CipherCtx::new()?;

        ctx.decrypt_init(self.cipher.to_openssl(), Some(key), Some(iv))?;
        ctx.set_padding(false);

        if let Some(buf) = aad {
            ctx.cipher_update(buf, None)?;
        }

        self.outp.resize(ptext_bytes, 0);
        ctx.cipher_update(
            &self.inp[..ctext_bytes],
            Some(&mut self.outp[..ptext_bytes]),
        )?;

        if self.cipher.tag_size() > 0 {
            ctx.set_tag(&self.inp[ctext_bytes..])?;
            ctx.cipher_final(&mut [])
                .map_err(|_| CipherError::NotTrustworthy)?;
        }

        Ok(ctext_bytes)
    }

    fn make_none(&mut self) {
        self.outp.clear();
        self.outp.extend_from_slice(&self.inp);
    }
}
