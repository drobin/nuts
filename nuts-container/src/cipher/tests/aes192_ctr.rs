// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use crate::cipher::{Cipher, CipherContext, CipherError};

use super::{ctx_test, IV, KEY};

const KEY_LEN: usize = 24;

#[test]
fn block_size() {
    assert_eq!(Cipher::Aes192Ctr.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::Aes192Ctr.key_len(), KEY_LEN);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::Aes192Ctr.iv_len(), 16);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::Aes192Ctr.tag_size(), 0);
}

#[test]
fn ctx_decrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes192Ctr);

    ctx.copy_from_slice(3, &[146, 140, 10]);

    let err = ctx.decrypt(&KEY[..KEY_LEN - 1], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_decrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes192Ctr);

    ctx.copy_from_slice(3, &[146, 140, 10]);

    let err = ctx.decrypt(&KEY[..KEY_LEN], &IV[..15]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(ctx_decrypt_3_1, Aes192Ctr.decrypt, 3, [85, 128, 31] -> [1, 2, 3]);
ctx_test!(ctx_decrypt_3_2, Aes192Ctr.decrypt, 2, [85, 128, 31] -> [1, 2]);
ctx_test!(ctx_decrypt_3_3, Aes192Ctr.decrypt, 4, [85, 128, 31] -> [1, 2, 3, 242]);
ctx_test!(ctx_decrypt_2_1, Aes192Ctr.decrypt, 2, [85, 128] -> [1, 2]);
ctx_test!(ctx_decrypt_2_2, Aes192Ctr.decrypt, 1, [85, 128] -> [1]);
ctx_test!(ctx_decrypt_2_3, Aes192Ctr.decrypt, 3, [85, 128] -> [1, 2, 28]);
ctx_test!(ctx_decrypt_1_1, Aes192Ctr.decrypt, 1, [85] -> [1]);
ctx_test!(ctx_decrypt_1_2, Aes192Ctr.decrypt, 0, [85] -> []);
ctx_test!(ctx_decrypt_1_3, Aes192Ctr.decrypt, 2, [85] -> [1, 130]);
ctx_test!(ctx_decrypt_0_1, Aes192Ctr.decrypt, 0, [] -> []);
ctx_test!(ctx_decrypt_0_2, Aes192Ctr.decrypt, 1, [] -> [84]);

#[test]
fn ctx_encrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes192Ctr);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN - 1], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_encrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes192Ctr);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN], &IV[..15]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(ctx_encrypt_3_1, Aes192Ctr.encrypt, 3, [1, 2, 3] -> [85, 128, 31]);
ctx_test!(ctx_encrypt_3_2, Aes192Ctr.encrypt, 2, [1, 2, 3] -> [85, 128]);
ctx_test!(ctx_encrypt_3_3, Aes192Ctr.encrypt, 4, [1, 2, 3] -> [85, 128, 31, 242]);
ctx_test!(ctx_encrypt_2_1, Aes192Ctr.encrypt, 2, [1, 2] -> [85, 128]);
ctx_test!(ctx_encrypt_2_2, Aes192Ctr.encrypt, 1, [1, 2] -> [85]);
ctx_test!(ctx_encrypt_2_3, Aes192Ctr.encrypt, 3, [1, 2] -> [85, 128, 28]);
ctx_test!(ctx_encrypt_1_1, Aes192Ctr.encrypt, 1, [1] -> [85]);
ctx_test!(ctx_encrypt_1_2, Aes192Ctr.encrypt, 0, [1] -> []);
ctx_test!(ctx_encrypt_1_3, Aes192Ctr.encrypt, 2, [1] -> [85, 130]);
ctx_test!(ctx_encrypt_0_1, Aes192Ctr.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, Aes192Ctr.encrypt, 1, [] -> [84]);
