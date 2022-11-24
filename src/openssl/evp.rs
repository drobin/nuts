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

use std::os::raw::{c_char, c_int, c_uchar};
use std::ptr;

use crate::openssl::error::{OpenSSLError, OpenSSLResult};
use crate::openssl::{MapResult, MapResultPtr};

#[allow(non_camel_case_types)]
enum EVP_CIPHER {}
#[allow(non_camel_case_types)]
enum EVP_CIPHER_CTX {}
enum ENGINE {}
#[allow(non_camel_case_types)]
enum EVP_MD {}

extern "C" {
    fn EVP_aes_128_ctr() -> *const EVP_CIPHER;

    fn EVP_sha1() -> *const EVP_MD;

    fn EVP_CIPHER_block_size(e: *const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_key_length(e: *const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_iv_length(e: *const EVP_CIPHER) -> c_int;

    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_reset(ctx: *mut EVP_CIPHER_CTX) -> c_int;
    fn EVP_CIPHER_CTX_free(ctx: *mut EVP_CIPHER_CTX);
    fn EVP_CIPHER_CTX_block_size(ctx: *const EVP_CIPHER_CTX) -> c_int;
    fn EVP_CIPHER_CTX_set_padding(x: *mut EVP_CIPHER_CTX, padding: c_int) -> c_int;

    fn EVP_CipherInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        r#type: *const EVP_CIPHER,
        r#impl: *mut ENGINE,
        key: *const c_uchar,
        iv: *const c_uchar,
        enc: c_int,
    ) -> c_int;
    fn EVP_CipherUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut c_uchar,
        outl: *mut c_int,
        r#in: *const c_uchar,
        inl: c_int,
    ) -> c_int;

    fn PKCS5_PBKDF2_HMAC(
        pass: *const c_char,
        passlen: c_int,
        salt: *const c_uchar,
        saltlen: c_int,
        iter: c_int,
        digest: *const EVP_MD,
        keylen: c_int,
        out: *mut c_uchar,
    ) -> c_int;
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

pub struct CipherCtx(*mut EVP_CIPHER_CTX);

impl CipherCtx {
    pub fn new() -> OpenSSLResult<CipherCtx> {
        unsafe { EVP_CIPHER_CTX_new() }.map_result(|ctx| CipherCtx(ctx))
    }

    unsafe fn init(
        &mut self,
        cipher: Cipher,
        enc: c_int,
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

        EVP_CIPHER_CTX_reset(self.0).into_result()?;
        EVP_CipherInit_ex(self.0, cipher.0, ptr::null_mut(), key, iv, enc).into_result()?;
        EVP_CIPHER_CTX_set_padding(self.0, 0).into_result()?;

        Ok(())
    }

    unsafe fn update(&mut self, inp: &[u8], outp: &mut [u8]) -> OpenSSLResult<usize> {
        let block_size = EVP_CIPHER_CTX_block_size(self.0) as usize;

        if inp.len() % block_size != 0 {
            return Err(OpenSSLError::InvalidBlockSize);
        }

        let outl = outp.len();

        EVP_CipherUpdate(
            self.0,
            outp.as_mut_ptr(),
            &mut (outl as c_int),
            inp.as_ptr(),
            inp.len() as c_int,
        )
        .into_result()?;

        Ok(outl)
    }

    pub fn encrypt(
        &mut self,
        cipher: Cipher,
        key: &[u8],
        iv: &[u8],
        inp: &[u8],
        outp: &mut [u8],
    ) -> OpenSSLResult<usize> {
        unsafe {
            self.init(cipher, 1, key, iv)
                .and_then(|()| self.update(inp, outp))
        }
    }

    pub fn decrypt(
        &mut self,
        cipher: Cipher,
        key: &[u8],
        iv: &[u8],
        inp: &[u8],
        outp: &mut [u8],
    ) -> OpenSSLResult<usize> {
        unsafe {
            self.init(cipher, 0, key, iv)
                .and_then(|()| self.update(inp, outp))
        }
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
