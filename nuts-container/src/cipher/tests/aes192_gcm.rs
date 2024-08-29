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
    assert_eq!(Cipher::Aes192Gcm.block_size(), 1);
}

#[test]
fn key_len() {
    assert_eq!(Cipher::Aes192Gcm.key_len(), KEY_LEN);
}

#[test]
fn iv_len() {
    assert_eq!(Cipher::Aes192Gcm.iv_len(), 12);
}

#[test]
fn tag_size() {
    assert_eq!(Cipher::Aes192Gcm.tag_size(), 16);
}

#[test]
fn ctx_decrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes192Gcm);

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
    let mut ctx = CipherContext::new(Cipher::Aes192Gcm);

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
    let mut ctx = CipherContext::new(Cipher::Aes192Gcm);

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
    ctx_decrypt_3, Aes192Gcm.decrypt,
    19, [220, 175, 49, 151, 5, 123, 186, 67, 30, 150, 27, 255, 127, 1, 161, 242, 179, 112, 247] -> [1, 2, 3]
);
ctx_test!(
    ctx_decrypt_2, Aes192Gcm.decrypt,
    18, [220, 175, 220, 189, 178, 162, 226, 222, 90, 251, 231, 62, 227, 253, 243, 228, 251, 189] -> [1, 2]
);
ctx_test!(
    ctx_decrypt_1, Aes192Gcm.decrypt,
    17, [220, 123, 0, 76, 124, 12, 194, 91, 115, 73, 46, 169, 28, 110, 229, 91, 33] -> [1]
);
ctx_test!(
    ctx_decrypt_0_1, Aes192Gcm.decrypt,
    16, [113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(
    ctx_decrypt_0_2, Aes192Gcm.decrypt,
    15, [88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213] -> []
);
ctx_test!(ctx_decrypt_0_3, Aes192Gcm.decrypt, 0, [] -> []);

#[test]
fn ctx_encrypt_inval_key() {
    let mut ctx = CipherContext::new(Cipher::Aes192Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN - 1], &IV).unwrap_err();
    assert!(matches!(err, CipherError::InvalidKey));
}

#[test]
fn ctx_encrypt_inval_iv() {
    let mut ctx = CipherContext::new(Cipher::Aes192Gcm);

    ctx.copy_from_slice(3, &[1, 2, 3]);

    let err = ctx.encrypt(&KEY[..KEY_LEN], &IV[..11]).unwrap_err();
    assert!(matches!(err, CipherError::InvalidIv));
}

ctx_test!(
    ctx_encrypt_3_1, Aes192Gcm.encrypt,
    3, [1, 2, 3] -> [220, 175, 49, 151, 5, 123, 186, 67, 30, 150, 27, 255, 127, 1, 161, 242, 179, 112, 247]
);
ctx_test!(
    ctx_encrypt_3_2, Aes192Gcm.encrypt,
    2, [1, 2, 3] -> [220, 175, 220, 189, 178, 162, 226, 222, 90, 251, 231, 62, 227, 253, 243, 228, 251, 189]
);
ctx_test!(
    ctx_encrypt_3_3, Aes192Gcm.encrypt,
    4, [1, 2, 3] -> [220, 175, 49, 56, 59, 114, 129, 57, 70, 102, 49, 121, 149, 159, 245, 249, 203, 58, 146, 59]
);
ctx_test!(
    ctx_encrypt_2_1, Aes192Gcm.encrypt,
    2, [1, 2] -> [220, 175, 220, 189, 178, 162, 226, 222, 90, 251, 231, 62, 227, 253, 243, 228, 251, 189]
);
ctx_test!(
    ctx_encrypt_2_2, Aes192Gcm.encrypt,
    1, [1, 2] -> [220, 123, 0, 76, 124, 12, 194, 91, 115, 73, 46, 169, 28, 110, 229, 91, 33]
);
ctx_test!(
    ctx_encrypt_2_3, Aes192Gcm.encrypt,
    3, [1, 2] -> [220, 175, 50, 40, 167, 251, 97, 168, 35, 223, 99, 86, 132, 178, 27, 244, 180, 121, 12]
);
ctx_test!(
    ctx_encrypt_1_1, Aes192Gcm.encrypt,
    1, [1] -> [220, 123, 0, 76, 124, 12, 194, 91, 115, 73, 46, 169, 28, 110, 229, 91, 33]
);
ctx_test!(
    ctx_encrypt_1_2, Aes192Gcm.encrypt,
    0, [1] -> []
);
ctx_test!(
    ctx_encrypt_1_3, Aes192Gcm.encrypt,
    2, [1] -> [220, 173, 246, 66, 223, 239, 52, 175, 245, 202, 181, 227, 207, 249, 246, 21, 169, 22]
);
ctx_test!(ctx_encrypt_0_1, Aes192Gcm.encrypt, 0, [] -> []);
ctx_test!(ctx_encrypt_0_2, Aes192Gcm.encrypt, 1, [] -> [221, 60, 182, 234, 151, 52, 21, 195, 218, 39, 184, 171, 30, 150, 76, 14, 189]);
