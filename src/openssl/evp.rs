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

use std::os::raw::c_int;

#[allow(non_camel_case_types)]
enum EVP_CIPHER {}

extern "C" {
    fn EVP_aes_128_ctr() -> *const EVP_CIPHER;

    fn EVP_CIPHER_block_size(e: *const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_key_length(e: *const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_iv_length(e: *const EVP_CIPHER) -> c_int;
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
