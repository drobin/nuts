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

use openssl_sys::{
    EVP_CIPHER_CTX_block_size, EVP_CIPHER_CTX_free, EVP_CIPHER_CTX_new, EVP_CIPHER_CTX_set_padding,
    EVP_CIPHER_block_size, EVP_CIPHER_iv_length, EVP_CIPHER_key_length, EVP_CipherInit_ex,
    EVP_CipherUpdate, EVP_aes_128_ctr, EVP_sha1, EVP_CIPHER, EVP_CIPHER_CTX, EVP_MD,
    PKCS5_PBKDF2_HMAC,
};
use std::os::raw::c_int;
use std::ptr;

use crate::container::ossl::error::{OpenSSLError, OpenSSLResult};
use crate::container::ossl::{MapResult, MapResultPtr};

extern "C" {
    fn EVP_CIPHER_CTX_reset(ctx: *mut EVP_CIPHER_CTX) -> c_int;
}

pub struct Cipher(*const EVP_CIPHER);

impl Cipher {
    pub fn aes128_ctr() -> Cipher {
        Cipher(unsafe { EVP_aes_128_ctr() })
    }

    pub fn block_size(&self) -> usize {
        let n = unsafe { EVP_CIPHER_block_size(self.0) };
        n as usize
    }

    pub fn key_length(&self) -> usize {
        let n = unsafe { EVP_CIPHER_key_length(self.0) };
        n as usize
    }

    pub fn iv_length(&self) -> usize {
        let n = unsafe { EVP_CIPHER_iv_length(self.0) };
        n as usize
    }
}

pub struct Digest(*const EVP_MD);

impl Digest {
    pub fn sha1() -> Digest {
        Digest(unsafe { EVP_sha1() })
    }
}

#[derive(Clone, Copy)]
pub enum CipherMode {
    Encrypt,
    Decrypt,
}

impl CipherMode {
    pub fn into_enc(self) -> c_int {
        match self {
            CipherMode::Encrypt => 1,
            CipherMode::Decrypt => 0,
        }
    }
}

pub struct CipherCtx(*mut EVP_CIPHER_CTX);

impl CipherCtx {
    pub fn new() -> OpenSSLResult<CipherCtx> {
        unsafe { EVP_CIPHER_CTX_new() }.map_result(|ctx| CipherCtx(ctx))
    }

    pub fn reset(&mut self) -> OpenSSLResult<()> {
        unsafe { EVP_CIPHER_CTX_reset(self.0) }.map_result(|_| ())
    }

    pub fn init(
        &self,
        cipher: Cipher,
        mode: CipherMode,
        key: &[u8],
        iv: &[u8],
    ) -> OpenSSLResult<()> {
        let key = key
            .get(..cipher.key_length())
            .ok_or(OpenSSLError::InvalidKey)?
            .as_ptr();
        let iv = iv
            .get(..cipher.iv_length())
            .ok_or(OpenSSLError::InvalidIv)?
            .as_ptr();

        unsafe {
            EVP_CipherInit_ex(self.0, cipher.0, ptr::null_mut(), key, iv, mode.into_enc())
                .into_result()
                .and_then(|_| EVP_CIPHER_CTX_set_padding(self.0, 0).map_result(|_| ()))
        }
    }

    pub fn update(&mut self, inp: &[u8], outp: &mut [u8]) -> OpenSSLResult<usize> {
        let block_size = unsafe { EVP_CIPHER_CTX_block_size(self.0) } as usize;

        if inp.len() % block_size != 0 {
            return Err(OpenSSLError::InvalidBlockSize);
        }

        let outl = outp.len();

        unsafe {
            EVP_CipherUpdate(
                self.0,
                outp.as_mut_ptr(),
                &mut (outl as c_int),
                inp.as_ptr(),
                inp.len() as c_int,
            )
        }
        .map_result(|_| outl)
    }
}

impl Drop for CipherCtx {
    fn drop(&mut self) {
        unsafe { EVP_CIPHER_CTX_free(self.0) }
    }
}

pub fn pbkdf2_hmac(
    pass: &[u8],
    salt: &[u8],
    iter: u32,
    digest: Digest,
    key: &mut [u8],
) -> OpenSSLResult<()> {
    unsafe {
        PKCS5_PBKDF2_HMAC(
            pass.as_ptr() as *const i8,
            pass.len() as c_int,
            salt.as_ptr(),
            salt.len() as c_int,
            iter as c_int,
            digest.0,
            key.len() as c_int,
            key.as_mut_ptr(),
        )
    }
    .map_result(|_| ())
}
