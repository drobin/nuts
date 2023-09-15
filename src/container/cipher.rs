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

use openssl::cipher as ossl_cipher;
use openssl::cipher_ctx as ossl_cipherctx;
use openssl::error as ossl_error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{cmp, error, fmt};

use crate::container::ossl::{evp, OpenSSLError};
use crate::container::svec::SecureVec;

/// An error which can be returned when parsing a [`Cipher`].
///
/// This error is used as the error type for the [`FromStr`] implementation for
/// [`Cipher`].
#[derive(Debug)]
pub enum CipherError {
    OpenSSL(ossl_error::ErrorStack),
    InvalidKey,
    InvalidIv,
    InvalidBlockSize,
    Invalid(String),
}

impl fmt::Display for CipherError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
            Self::InvalidKey => write!(fmt, "Invalid key"),
            Self::InvalidIv => write!(fmt, "Invalid iv"),
            Self::InvalidBlockSize => write!(fmt, "Invalid block-size"),
            Self::Invalid(str) => write!(fmt, "invalid cipher: {}", str),
        }
    }
}

impl error::Error for CipherError {}

impl From<ossl_error::ErrorStack> for CipherError {
    fn from(cause: ossl_error::ErrorStack) -> Self {
        CipherError::OpenSSL(cause)
    }
}

/// Supported cipher algorithms.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn tag_size(&self) -> usize {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 0,
            Cipher::Aes128Gcm => 16,
        }
    }

    pub fn encrypt(
        &self,
        input: &[u8],
        output: &mut Vec<u8>,
        key: &[u8],
        iv: &[u8],
    ) -> Result<usize, CipherError> {
        match self {
            Self::None => Ok(Self::make_none(input, output)),
            _ => self.encrypt_aad(None, input, output, key, iv),
        }
    }

    fn make_none(input: &[u8], output: &mut Vec<u8>) -> usize {
        output.clear();
        output.extend_from_slice(input);

        input.len()
    }

    fn encrypt_aad(
        &self,
        aad: Option<&[u8]>,
        input: &[u8],
        output: &mut Vec<u8>,
        key: &[u8],
        iv: &[u8],
    ) -> Result<usize, CipherError> {
        let key = key.get(..self.key_len()).ok_or(CipherError::InvalidKey)?;
        let iv = iv.get(..self.iv_len()).ok_or(CipherError::InvalidIv)?;

        // number of plaintext bytes: equals to number of input bytes. There is
        // no need to align at block size because blocksize is 1 for all
        // ciphers.
        let ptext_len = input.len();

        // number of ciphertext bytes: equals to plaintext bytes (for now) because
        // blocksize is 1 for all ciphers.
        let ctext_len = ptext_len;

        if ptext_len == 0 {
            return Ok(0);
        }

        if ptext_len % self.block_size() != 0 {
            return Err(CipherError::InvalidBlockSize);
        }

        let mut ctx = ossl_cipherctx::CipherCtx::new()?;

        ctx.encrypt_init(self.to_openssl(), Some(key), Some(iv))?;
        ctx.set_padding(false);

        if let Some(buf) = aad {
            ctx.cipher_update(buf, None)?;
        }

        output.resize(ctext_len + self.tag_size(), 0);
        ctx.cipher_update(&input[..ptext_len], Some(&mut output[..ctext_len]))?;

        if self.tag_size() > 0 {
            ctx.cipher_final(&mut [])?;
            ctx.tag(&mut output[ctext_len..])?;
        }

        Ok(ctext_len)
    }

    fn to_evp(&self) -> Option<evp::Cipher> {
        match self {
            Cipher::None => None,
            Cipher::Aes128Ctr => Some(evp::Cipher::aes128_ctr()),
            Cipher::Aes128Gcm => unimplemented!(),
        }
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
    type Err = CipherError;

    fn from_str(str: &str) -> Result<Self, CipherError> {
        match str {
            "none" => Ok(Cipher::None),
            "aes128-ctr" => Ok(Cipher::Aes128Ctr),
            "aes128-gcm" => Ok(Cipher::Aes128Gcm),
            _ => Err(CipherError::Invalid(str.to_string())),
        }
    }
}

pub struct CipherCtx {
    ctx: evp::CipherCtx,
    cipher: Cipher,
    out: SecureVec,
}

impl CipherCtx {
    pub fn new(cipher: Cipher) -> Result<CipherCtx, OpenSSLError> {
        Ok(CipherCtx {
            ctx: evp::CipherCtx::new()?,
            cipher,
            out: vec![].into(),
        })
    }

    pub fn encrypt(&mut self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<&[u8], OpenSSLError> {
        match self.cipher.to_evp() {
            Some(cipher) => self.update_some(cipher, evp::CipherMode::Encrypt, key, iv, input),
            None => self.update_none(input),
        }
    }

    pub fn decrypt(&mut self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<&[u8], OpenSSLError> {
        match self.cipher.to_evp() {
            Some(cipher) => self.update_some(cipher, evp::CipherMode::Decrypt, key, iv, input),
            None => self.update_none(input),
        }
    }

    fn update_some(
        &mut self,
        cipher: evp::Cipher,
        mode: evp::CipherMode,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
    ) -> Result<&[u8], OpenSSLError> {
        self.out.resize(input.len(), 0);

        self.ctx.reset()?;
        self.ctx.init(cipher, mode, key, iv)?;
        let n = self.ctx.update(input, &mut self.out)?;

        self.out.resize(n, 0);

        Ok(&self.out)
    }

    fn update_none(&mut self, input: &[u8]) -> Result<&[u8], OpenSSLError> {
        self.out.clear();
        self.out.extend_from_slice(input);

        Ok(&self.out)
    }
}
