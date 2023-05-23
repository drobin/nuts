// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use nuts_bytes::Options;

use crate::container::cipher::Cipher;
use crate::container::digest::Digest;
use crate::container::header::rev0;
use crate::container::header::secret::Secret;
use crate::container::kdf::Kdf;

#[test]
fn de_none() {
    let rev0 = Options::new()
        .from_bytes::<rev0::Data>(&[
            0x00, 0x00, 0x00, 0x00, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
            0x00, 0x00, 0x00, 0x00, // kdf
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
        ])
        .unwrap();

    assert_eq!(rev0.cipher, Cipher::None);
    assert_eq!(rev0.iv, []);
    assert_eq!(rev0.kdf, Kdf::None);
    assert_eq!(rev0.secret, [1, 2, 3]);
}

#[test]
fn de_some() {
    let rev0 = Options::new()
        .from_bytes::<rev0::Data>(&[
            0x00, 0x00, 0x00, 0x01, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 1, 2, // iv,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x000, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 3, 4, 5, // kdf
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 6, 7, 8, 9, // secret
        ])
        .unwrap();

    assert_eq!(rev0.cipher, Cipher::Aes128Ctr);
    assert_eq!(rev0.iv, [1, 2]);
    assert_eq!(
        rev0.kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![3, 4, 5],
        }
    );
    assert_eq!(rev0.secret, [6, 7, 8, 9]);
}

#[test]
fn ser_none() {
    let rev0 = rev0::Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: Secret::new(vec![1, 2, 3]),
    };

    let vec = Options::new().to_vec(&rev0).unwrap();
    assert_eq!(
        vec,
        [
            0x00, 0x00, 0x00, 0x00, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
            0x00, 0x00, 0x00, 0x00, // kdf
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
        ]
    );
}

#[test]
fn ser_some() {
    let rev0 = rev0::Data {
        cipher: Cipher::Aes128Ctr,
        iv: vec![1, 2],
        kdf: Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![3, 4, 5],
        },
        secret: Secret::new(vec![6, 7, 8, 9]),
    };

    let vec = Options::new().to_vec(&rev0).unwrap();
    assert_eq!(
        vec,
        [
            0x00, 0x00, 0x00, 0x01, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 1, 2, // iv,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x000, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 3, 4, 5, // kdf
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 6, 7, 8, 9, // secret
        ]
    );
}
