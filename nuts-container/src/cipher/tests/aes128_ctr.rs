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

use crate::cipher::{Cipher, CipherContext, CipherError};

use super::{ctx_test, IV, KEY};

#[test]
fn block_size() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::Aes128Ctr.key_len(), 16);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::Aes128Ctr.iv_len(), 16);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::Aes128Ctr.tag_size(), 0);
}

#[test]
fn ctx_decrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes128Ctr);

    ctx.copy_from_slice(3, &[146, 140, 10]);

    let err = ctx.decrypt(&KEY[..15], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_decrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes128Ctr);

    ctx.copy_from_slice(3, &[146, 140, 10]);

    let err = ctx.decrypt(&KEY, &IV[..15]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(ctx_decrypt_3_1, Aes128Ctr.decrypt, 3, [146, 140, 10] -> [1, 2, 3]);
ctx_test!(ctx_decrypt_3_2, Aes128Ctr.decrypt, 2, [146, 140, 10] -> [1, 2]);
ctx_test!(ctx_decrypt_3_3, Aes128Ctr.decrypt, 4, [146, 140, 10] -> [1, 2, 3, 195]);
ctx_test!(ctx_decrypt_2_1, Aes128Ctr.decrypt, 2, [146, 140] -> [1, 2]);
ctx_test!(ctx_decrypt_2_2, Aes128Ctr.decrypt, 1, [146, 140] -> [1]);
ctx_test!(ctx_decrypt_2_3, Aes128Ctr.decrypt, 3, [146, 140] -> [1, 2, 9]);
ctx_test!(ctx_decrypt_1_1, Aes128Ctr.decrypt, 1, [146] -> [1]);
ctx_test!(ctx_decrypt_1_2, Aes128Ctr.decrypt, 0, [146] -> []);
ctx_test!(ctx_decrypt_1_3, Aes128Ctr.decrypt, 2, [146] -> [1, 142]);
ctx_test!(ctx_decrypt_0_1, Aes128Ctr.decrypt, 0, [] -> []);
ctx_test!(ctx_decrypt_0_2, Aes128Ctr.decrypt, 1, [] -> [147]);

#[test]
fn ctx_encrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes128Ctr);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..15], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_encrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes128Ctr);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY, &IV[..15]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(ctx_encrypt_3_1, Aes128Ctr.encrypt, 3, [1, 2, 3] -> [146, 140, 10]);
ctx_test!(ctx_encrypt_3_2, Aes128Ctr.encrypt, 2, [1, 2, 3] -> [146, 140]);
ctx_test!(ctx_encrypt_3_3, Aes128Ctr.encrypt, 4, [1, 2, 3] -> [146, 140, 10, 195]);
ctx_test!(ctx_encrypt_2_1, Aes128Ctr.encrypt, 2, [1, 2] -> [146, 140]);
ctx_test!(ctx_encrypt_2_2, Aes128Ctr.encrypt, 1, [1, 2] -> [146]);
ctx_test!(ctx_encrypt_2_3, Aes128Ctr.encrypt, 3, [1, 2] -> [146, 140, 9]);
ctx_test!(ctx_encrypt_1_1, Aes128Ctr.encrypt, 1, [1] -> [146]);
ctx_test!(ctx_encrypt_1_2, Aes128Ctr.encrypt, 0, [1] -> []);
ctx_test!(ctx_encrypt_1_3, Aes128Ctr.encrypt, 2, [1] -> [146, 142]);
ctx_test!(ctx_encrypt_0_1, Aes128Ctr.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, Aes128Ctr.encrypt, 1, [] -> [147]);
