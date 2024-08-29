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

const KEY_LEN: usize = 32;

#[test]
fn block_size() {
    assert_eq!(Cipher::Aes256Gcm.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::Aes256Gcm.key_len(), KEY_LEN);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::Aes256Gcm.iv_len(), 12);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::Aes256Gcm.tag_size(), 16);
}

#[test]
fn ctx_decrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes256Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
        ],
    );

    let err = ctx.decrypt(&KEY[..KEY_LEN - 1], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_decrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes256Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
        ],
    );

    let err = ctx.decrypt(&KEY[..KEY_LEN], &IV[..11]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

#[test]
fn ctx_decrypt_not_trustworthy() {
    let mut ctx = CipherContext::new(Cipher::Aes256Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, b'x',
        ],
    );

    let err = ctx.decrypt(&KEY[..KEY_LEN], &IV).unwrap_err();
    assert!(matches!(err, CipherError::NotTrustworthy));
}

ctx_test!(
    ctx_decrypt_3, Aes256Gcm.decrypt,
    19, [88, 192, 147, 60, 12, 126, 9, 47, 97, 73, 216, 21, 163, 162, 36, 191, 49, 206, 191] -> [1, 2, 3]
);
ctx_test!(
    ctx_decrypt_2, Aes256Gcm.decrypt,
    18, [88, 192, 242, 75, 64, 15, 81, 93, 138, 206, 22, 138, 83, 55, 47, 161, 154, 253] -> [1, 2]
);
ctx_test!(
    ctx_decrypt_1, Aes256Gcm.decrypt,
    17, [88, 200, 231, 95, 151, 3, 173, 184, 111, 242, 193, 7, 138, 44, 219, 174, 208] -> [1]
);
ctx_test!(
    ctx_decrypt_0_1, Aes256Gcm.decrypt,
    16, [113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(
    ctx_decrypt_0_2, Aes256Gcm.decrypt,
    15, [88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(ctx_decrypt_0_3, Aes256Gcm.decrypt, 0, [] -> []);

#[test]
fn ctx_encrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes256Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN - 1], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_encrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes256Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN], &IV[..11]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(
    ctx_encrypt_3_1, Aes256Gcm.encrypt,
    3, [1, 2, 3] -> [88, 192, 147, 60, 12, 126, 9, 47, 97, 73, 216, 21, 163, 162, 36, 191, 49, 206, 191]
);
ctx_test!(
    ctx_encrypt_3_2, Aes256Gcm.encrypt,
    2, [1, 2, 3] -> [88, 192, 242, 75, 64, 15, 81, 93, 138, 206, 22, 138, 83, 55, 47, 161, 154, 253]
);
ctx_test!(
    ctx_encrypt_3_3, Aes256Gcm.encrypt,
    4, [1, 2, 3] -> [88, 192, 147, 165, 43, 112, 211, 86, 75, 171, 9, 78, 90, 55, 216, 115, 60, 41, 236, 158]
);
ctx_test!(
    ctx_encrypt_2_1, Aes256Gcm.encrypt,
    2, [1, 2] -> [88, 192, 242, 75, 64, 15, 81, 93, 138, 206, 22, 138, 83, 55, 47, 161, 154, 253]
);
ctx_test!(
    ctx_encrypt_2_2, Aes256Gcm.encrypt,
    1, [1, 2] -> [88, 200, 231, 95, 151, 3, 173, 184, 111, 242, 193, 7, 138, 44, 219, 174, 208]
);
ctx_test!(
    ctx_encrypt_2_3, Aes256Gcm.encrypt,
    3, [1, 2] -> [88, 192, 144, 101, 143, 242, 117, 237, 138, 123, 81, 37, 141, 49, 172, 129, 5, 233, 162]
);
ctx_test!(
    ctx_encrypt_1_1, Aes256Gcm.encrypt,
    1, [1] -> [88, 200, 231, 95, 151, 3, 173, 184, 111, 242, 193, 7, 138, 44, 219, 174, 208]
);
ctx_test!(
    ctx_encrypt_1_2, Aes256Gcm.encrypt,
    0, [1] -> []
);
ctx_test!(
    ctx_encrypt_1_3, Aes256Gcm.encrypt,
    2, [1] -> [88, 194, 233, 67, 23, 115, 227, 126, 132, 238, 34, 104, 163, 28, 247, 155, 140, 174]
);
ctx_test!(ctx_encrypt_0_1, Aes256Gcm.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, Aes256Gcm.encrypt, 1, [] -> [89, 216, 204, 225, 206, 18, 42, 168, 117, 131, 185, 18, 102, 49, 208, 135, 90]);
