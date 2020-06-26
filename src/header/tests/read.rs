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
use crate::types::{Cipher, Digest, WrappingKey};
use crate::wkey::WrappingKeyData;

struct Data {
    magic: [u8; 7],
    revision: u8,
    cipher: u8,
    digest: u8,
    wkey: [u8; 16],
    wkey_size: usize,
    hmac: [u8; 8],
    hmac_size: usize,
    secret: [u8; 8],
    secret_size: usize,
}

const OK_DATA: Data = Data {
    magic: [b'n', b'u', b't', b's', b'-', b'i', b'o'],
    revision: 1,
    cipher: 1,
    digest: 1,
    wkey: [1, 0, 0, 0x12, 0x67, 0, 0, 0, 0x07, 1, 2, 3, 4, 5, 6, 7],
    wkey_size: 16,
    hmac: [0, 0, 0, 3, 1, 2, 3, 0],
    hmac_size: 7,
    secret: [0, 0, 0, 4, 4, 5, 6, 7],
    secret_size: 8,
};

fn mk_data(d: &Data) -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&d.magic);
    data.push(d.revision);
    data.push(d.cipher);
    data.push(d.digest);
    data.extend_from_slice(&d.wkey[0..d.wkey_size]);
    data.extend_from_slice(&d.hmac[0..d.hmac_size]);
    data.extend_from_slice(&d.secret[0..d.secret_size]);

    data
}

fn mk_data_size(d: &Data, size: usize) -> Vec<u8> {
    let mut data = mk_data(d);

    data.resize(size, 0xFF);

    data
}

#[test]
fn ok() {
    let data = mk_data(&OK_DATA);
    let (header, offset) = Header::read(&data).unwrap();

    assert_eq!(offset, 41);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::Aes128Ctr);
    assert_eq!(header.digest, Some(Digest::Sha1));
    assert_eq!(
        header.wrapping_key,
        Some(WrappingKeyData {
            wkey: WrappingKey::Pbkdf2 {
                iterations: 4711,
                salt_len: 7
            },
            pbkdf2: Some(vec![1, 2, 3, 4, 5, 6, 7])
        })
    );
    assert_eq!(header.hmac, vec![1, 2, 3]);
    assert_eq!(header.secret, vec![4, 5, 6, 7]);
}

#[test]
fn incomplete() {
    for i in 1..41 {
        let data = mk_data_size(&OK_DATA, i);
        let err = format!("{:?}", Header::read(&data).unwrap_err());
        assert_eq!(err, "NoData");
    }
}

#[test]
fn bad_magic() {
    let data = mk_data(&Data {
        magic: [b'X', b'u', b't', b's', b'-', b'i', b'o'],
        ..OK_DATA
    });

    let err = format!("{:?}", Header::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalMagic)");
}

#[test]
fn bad_revision() {
    let data = mk_data(&Data {
        revision: 0,
        ..OK_DATA
    });

    let err = format!("{:?}", Header::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalRevision)");
}

#[test]
fn bad_cipher() {
    let data = mk_data(&Data {
        cipher: 99,
        ..OK_DATA
    });

    let err = format!("{:?}", Header::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalCipher)");
}

#[test]
fn bad_digest() {
    let data = mk_data(&Data {
        digest: 99,
        ..OK_DATA
    });

    let err = format!("{:?}", Header::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalDigest)");
}

#[test]
fn bad_wrapping_key() {
    let data = mk_data(&Data {
        wkey: [9, 0, 0, 0x12, 0x67, 0, 0, 0, 0x07, 1, 2, 3, 4, 5, 6, 7],
        ..OK_DATA
    });

    let err = format!("{:?}", Header::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalWrappingKey)");
}
