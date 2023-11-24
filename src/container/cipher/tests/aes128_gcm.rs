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

use crate::container::cipher::{Cipher, CipherContext, CipherError};

use super::{ctx_test, IV, KEY};

#[test]
fn block_size() {
    assert_eq!(Cipher::Aes128Gcm.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::Aes128Gcm.key_len(), 16);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::Aes128Gcm.iv_len(), 12);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::Aes128Gcm.tag_size(), 16);
}

#[test]
fn ctx_decrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes128Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
        ],
    );

    let err = ctx.decrypt(&KEY[..15], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_decrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes128Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
        ],
    );

    let err = ctx.decrypt(&KEY, &IV[..11]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

#[test]
fn ctx_decrypt_not_trustworthy() {
    let mut ctx = CipherContext::new(Cipher::Aes128Gcm);

    ctx.copy_from_slice(
        19,
        &[
            143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, b'x',
        ],
    );

    let err = ctx.decrypt(&KEY, &IV).unwrap_err();
    assert!(matches!(err, CipherError::NotTrustworthy));
}

ctx_test!(
    ctx_decrypt_3, Aes128Gcm.decrypt,
    19, [143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91] -> [1, 2, 3]
);
ctx_test!(
    ctx_decrypt_2, Aes128Gcm.decrypt,
    18, [143, 103, 41, 94, 18, 247, 152, 74, 58, 41, 133, 131, 106, 84, 84, 135, 195, 243] -> [1, 2]
);
ctx_test!(
    ctx_decrypt_1, Aes128Gcm.decrypt,
    17, [143, 113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> [1]
);
ctx_test!(
    ctx_decrypt_0_1, Aes128Gcm.decrypt,
    16, [113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(
    ctx_decrypt_0_2, Aes128Gcm.decrypt,
    15, [88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(ctx_decrypt_0_3, Aes128Gcm.decrypt, 0, [] -> []);

#[test]
fn ctx_encrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes128Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..15], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_encrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes128Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY, &IV[..11]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(
    ctx_encrypt_3_1, Aes128Gcm.encrypt,
    3, [1, 2, 3] -> [143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91]
);
ctx_test!(
    ctx_encrypt_3_2, Aes128Gcm.encrypt,
    2, [1, 2, 3] -> [143, 103, 41, 94, 18, 247, 152, 74, 58, 41, 133, 131, 106, 84, 84, 135, 195, 243]
);
ctx_test!(
    ctx_encrypt_3_3, Aes128Gcm.encrypt,
    4, [1, 2, 3] -> [143, 103, 80, 25, 185, 55, 28, 172, 175, 38, 31, 213, 0, 239, 128, 175, 53, 160, 251, 121]
);
ctx_test!(
    ctx_encrypt_2_1, Aes128Gcm.encrypt,
    2, [1, 2] -> [143, 103, 41, 94, 18, 247, 152, 74, 58, 41, 133, 131, 106, 84, 84, 135, 195, 243]
);
ctx_test!(
    ctx_encrypt_2_2, Aes128Gcm.encrypt,
    1, [1, 2] -> [143, 113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213]
);
ctx_test!(
    ctx_encrypt_2_3, Aes128Gcm.encrypt,
    3, [1, 2] -> [143, 103, 83, 43, 1, 58, 109, 96, 18, 144, 41, 58, 65, 63, 245, 18, 60, 121, 153]
);
ctx_test!(
    ctx_encrypt_1_1, Aes128Gcm.encrypt,
    1, [1] -> [143, 113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213]
);
ctx_test!(
    ctx_encrypt_1_2, Aes128Gcm.encrypt,
    0, [1] -> []
);
ctx_test!(
    ctx_encrypt_1_3, Aes128Gcm.encrypt,
    2, [1] -> [143, 101, 66, 143, 158, 164, 113, 118, 91, 218, 10, 203, 55, 60, 167, 211, 64, 4]
);
ctx_test!(ctx_encrypt_0_1, Aes128Gcm.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, Aes128Gcm.encrypt, 1, [] -> [142, 77, 158, 210, 213, 221, 151, 217, 234, 130, 117, 125, 12, 137, 10, 45, 127]);
