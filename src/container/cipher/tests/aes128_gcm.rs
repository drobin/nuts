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

use crate::container::cipher::Cipher;
use crate::container::error::Error;
use crate::memory::MemoryBackend;

use super::{cipher_test, IV, KEY};

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
fn decrypt_inval_key() {
    let err = Cipher::Aes128Gcm
        .decrypt::<MemoryBackend>(
            &[
                143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
            ],
            &mut Vec::new(),
            &KEY[..15],
            &IV,
        )
        .unwrap_err();
    assert!(matches!(err, Error::InvalidKey));
}

#[test]
fn decrypt_inval_iv() {
    let err = Cipher::Aes128Gcm
        .decrypt::<MemoryBackend>(
            &[
                143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91,
            ],
            &mut Vec::new(),
            &KEY,
            &IV[..11],
        )
        .unwrap_err();
    assert!(matches!(err, Error::InvalidIv));
}

#[test]
fn decrypt_not_trustworthy() {
    let err = Cipher::Aes128Gcm
        .decrypt::<MemoryBackend>(
            &[
                143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, b'x',
            ],
            &mut Vec::new(),
            &KEY,
            &IV,
        )
        .unwrap_err();
    assert!(matches!(err, Error::NotTrustworthy));
}

cipher_test!(
    decrypt_1,
    Aes128Gcm.decrypt,
    [143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91],
    3,
    [1, 2, 3]
);
cipher_test!(
    decrypt_2,
    Aes128Gcm.decrypt,
    [143, 103, 41, 94, 18, 247, 152, 74, 58, 41, 133, 131, 106, 84, 84, 135, 195, 243],
    2,
    [1, 2]
);
cipher_test!(
    decrypt_3,
    Aes128Gcm.decrypt,
    [143, 113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213],
    1,
    [1]
);
cipher_test!(
    decrypt_4,
    Aes128Gcm.decrypt,
    [113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213],
    0,
    []
);
cipher_test!(
    decrypt_5,
    Aes128Gcm.decrypt,
    [88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213],
    0,
    []
);
cipher_test!(decrypt_6, Aes128Gcm.decrypt, [], 0, []);

#[test]
fn encrypt_inval_key() {
    let err = Cipher::Aes128Gcm
        .encrypt::<MemoryBackend>(&[1, 2, 3], &mut Vec::new(), &KEY[..15], &IV)
        .unwrap_err();
    assert!(matches!(err, Error::InvalidKey));
}

#[test]
fn encrypt_inval_iv() {
    let err = Cipher::Aes128Gcm
        .encrypt::<MemoryBackend>(&[1, 2, 3], &mut Vec::new(), &KEY, &IV[..11])
        .unwrap_err();
    assert!(matches!(err, Error::InvalidIv));
}

cipher_test!(
    encrypt_1,
    Aes128Gcm.encrypt,
    [1, 2, 3],
    3,
    [143, 103, 80, 34, 166, 3, 39, 26, 15, 50, 120, 48, 9, 211, 134, 206, 182, 135, 91]
);
cipher_test!(
    encrypt_2,
    Aes128Gcm.encrypt,
    [1, 2],
    2,
    [143, 103, 41, 94, 18, 247, 152, 74, 58, 41, 133, 131, 106, 84, 84, 135, 195, 243]
);
cipher_test!(
    encrypt_3,
    Aes128Gcm.encrypt,
    [1],
    1,
    [143, 113, 88, 251, 33, 67, 167, 32, 45, 38, 91, 201, 117, 35, 75, 214, 213]
);
cipher_test!(encrypt_4, Aes128Gcm.encrypt, [], 0, []);
