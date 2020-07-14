// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use crate::header::Header;
use crate::secret::Secret;
use crate::types::{Cipher, DiskType, Options};

#[test]
fn cipher_none_read_secret() {
    let options = Options::default_with_cipher(Cipher::None);
    let mut header = Header::create(&options).unwrap();

    header.secret = vec![
        1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 1, 0, 0, 0, 2, 2, 3, 0, 0, 0, 3, 4, 5,
        6, 0, 0, 0, 4, 7, 8, 9, 10,
    ];
    let (secret, offset) = header.read_secret(&[]).unwrap();

    assert_eq!(offset, 39);
    assert_eq!(secret.dtype, DiskType::FatRandom);
    assert_eq!(secret.bsize, 1);
    assert_eq!(secret.blocks, 2);
    assert_eq!(secret.master_key, vec![1]);
    assert_eq!(secret.master_iv, vec![2, 3]);
    assert_eq!(secret.hmac_key, vec![4, 5, 6]);
    assert_eq!(secret.userdata, vec![7, 8, 9, 10]);
}

#[test]
fn cipher_some_read_secret() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr);
    let mut header = Header::create(&options).unwrap();

    header.secret = vec![
        136, 152, 223, 191, 142, 167, 216, 122, 125, 85, 110, 171, 57, 38, 145, 46, 26, 142, 117,
        156, 127, 137, 138, 226, 174, 235, 54, 122, 81, 70, 245, 42, 145, 116, 101, 25, 189, 18,
        242,
    ];
    header.hmac = vec![
        151, 7, 172, 51, 96, 158, 20, 154, 100, 106, 21, 60, 235, 32, 150, 126, 51, 33, 142, 128,
    ];
    let (secret, offset) = header.read_secret(&[9; 16]).unwrap();

    assert_eq!(offset, 39);
    assert_eq!(secret.dtype, DiskType::FatRandom);
    assert_eq!(secret.bsize, 1);
    assert_eq!(secret.blocks, 2);
    assert_eq!(secret.master_key, vec![1]);
    assert_eq!(secret.master_iv, vec![2, 3]);
    assert_eq!(secret.hmac_key, vec![4, 5, 6]);
    assert_eq!(secret.userdata, vec![7, 8, 9, 10]);
}

#[test]
fn cipher_none_write_secret() {
    let options = Options::default_with_cipher(Cipher::None);
    let secret = Secret {
        dtype: DiskType::FatRandom,
        bsize: 1,
        blocks: 2,
        master_key: vec![1],
        master_iv: vec![2, 3],
        hmac_key: vec![4, 5, 6],
        userdata: vec![7, 8, 9, 10],
    };

    let mut header = Header::create(&options).unwrap();
    header.write_secret(&secret, &[]).unwrap();

    assert_eq!(
        header.secret,
        vec![
            1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 1, 0, 0, 0, 2, 2, 3, 0, 0, 0, 3, 4,
            5, 6, 0, 0, 0, 4, 7, 8, 9, 10,
        ]
    );
    assert_eq!(header.hmac, []);
}

#[test]
fn cipher_some_write_secret() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr);
    let secret = Secret {
        dtype: DiskType::FatRandom,
        bsize: 1,
        blocks: 2,
        master_key: vec![1],
        master_iv: vec![2, 3],
        hmac_key: vec![4, 5, 6],
        userdata: vec![7, 8, 9, 10],
    };

    let mut header = Header::create(&options).unwrap();
    header.write_secret(&secret, &[9; 16]).unwrap();

    assert_eq!(
        header.secret,
        vec![
            136, 152, 223, 191, 142, 167, 216, 122, 125, 85, 110, 171, 57, 38, 145, 46, 26, 142,
            117, 156, 127, 137, 138, 226, 174, 235, 54, 122, 81, 70, 245, 42, 145, 116, 101, 25,
            189, 18, 242,
        ]
    );
    assert_eq!(
        header.hmac,
        vec![
            151, 7, 172, 51, 96, 158, 20, 154, 100, 106, 21, 60, 235, 32, 150, 126, 51, 33, 142,
            128
        ]
    );
}
