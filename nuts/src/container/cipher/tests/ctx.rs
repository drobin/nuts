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

use crate::container::cipher::{Cipher, CipherCtx};
use crate::container::ContainerError;
use crate::memory::MemoryBackend as Backend;
use crate::openssl::OpenSSLError;

const KEY: [u8; 16] = [b'x'; 16];
const IV: [u8; 16] = [b'y'; 16];

#[test]
fn encrypt_none_with_key_iv() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 3).unwrap();

    let out = ctx.encrypt::<Backend>(&KEY, &IV, &[1, 2, 3]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn encrypt_none_without_key_iv() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 3).unwrap();

    let out = ctx.encrypt::<Backend>(&[], &[], &[1, 2, 3]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn encrypt_none_padded() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 4).unwrap();

    let out = ctx.encrypt::<Backend>(&[], &[], &[1, 2, 3]).unwrap();
    assert_eq!(out, [1, 2, 3, 0]);
}

#[test]
fn decrypt_none_with_key() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 3).unwrap();

    let out = ctx.decrypt::<Backend>(&KEY, &IV, &[1, 2, 3]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_none_without_key() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 3).unwrap();

    let out = ctx.decrypt::<Backend>(&[], &[], &[1, 2, 3]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_none_padded() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::None, 4).unwrap();

    let out = ctx.decrypt::<Backend>(&[], &[], &[1, 2, 3, 0]).unwrap();
    assert_eq!(out, [1, 2, 3, 0]);
}

#[test]
fn encrypt_aes128_ctr() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let out = ctx.encrypt::<Backend>(&KEY, &IV, &[1, 2, 3]).unwrap();
    assert_eq!(out, [146, 140, 10]);
}

#[test]
fn encrypt_aes128_ctr_padded() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 4).unwrap();

    let out = ctx.encrypt::<Backend>(&KEY, &IV, &[1, 2, 3]).unwrap();
    assert_eq!(out, [146, 140, 10, 195]);
}

#[test]
fn encrypt_aes128_ctr_inval_key() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let err = ctx
        .encrypt::<Backend>(&KEY[..15], &IV, &[1, 2, 3])
        .unwrap_err();
    let err = into_error!(err, ContainerError::OpenSSL);
    assert_error!(err, OpenSSLError::InvalidKey);
}

#[test]
fn encrypt_aes128_ctr_inval_iv() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let err = ctx
        .encrypt::<Backend>(&KEY, &IV[..15], &[1, 2, 3])
        .unwrap_err();
    let err = into_error!(err, ContainerError::OpenSSL);
    assert_error!(err, OpenSSLError::InvalidIv);
}

#[test]
fn decrypt_aes128_ctr() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let out = ctx.decrypt::<Backend>(&KEY, &IV, &[146, 140, 10]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_aes128_ctr_padded() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 4).unwrap();

    let out = ctx
        .decrypt::<Backend>(&KEY, &IV, &[146, 140, 10, 195])
        .unwrap();
    assert_eq!(out, [1, 2, 3, 0]);
}

#[test]
fn decrypt_aes128_ctr_inval_key() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let err = ctx
        .decrypt::<Backend>(&KEY[..15], &IV, &[146, 140, 10])
        .unwrap_err();
    let err = into_error!(err, ContainerError::OpenSSL);
    assert_error!(err, OpenSSLError::InvalidKey);
}

#[test]
fn decrypt_aes128_ctr_inval_iv() {
    let mut ctx = CipherCtx::new::<Backend>(Cipher::Aes128Ctr, 3).unwrap();

    let err = ctx
        .decrypt::<Backend>(&KEY, &IV[..15], &[146, 140, 10])
        .unwrap_err();
    let err = into_error!(err, ContainerError::OpenSSL);
    assert_error!(err, OpenSSLError::InvalidIv);
}