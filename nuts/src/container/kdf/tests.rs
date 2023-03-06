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

mod none;
mod pbkdf2;
mod serde;

use std::io::Cursor;

use nuts_bytes::{Error as BytesError, FromBytesExt, ToBytesExt};

use crate::container::digest::Digest;
use crate::container::kdf::Kdf;

#[test]
fn from_bytes_nospace() {
    let mut cursor = Cursor::new([]);
    let err = cursor.from_bytes::<Kdf>().unwrap_err();

    assert_error!(err, BytesError::Eof)
}

#[test]
fn from_bytes_inval() {
    let mut cursor = Cursor::new([2]);
    let err = cursor.from_bytes::<Kdf>().unwrap_err();

    let msg = into_error!(err, BytesError::Invalid);
    assert_eq!(msg, "invalid kdf: 2");
}

#[test]
fn from_bytes_none() {
    let mut cursor = Cursor::new([0]);
    let wkey = cursor.from_bytes::<Kdf>().unwrap();

    assert_eq!(wkey, Kdf::None);
}

#[test]
fn from_bytes_pbkdf2_nospace() {
    const DATA: [u8; 13] = [
        1, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
    ];

    for i in 0..DATA.len() {
        let mut cursor = Cursor::new(&DATA[..i]);
        let err = cursor.from_bytes::<Kdf>().unwrap_err();
        assert_error!(err, BytesError::Eof);
    }
}

#[test]
fn from_bytes_pbkdf2() {
    let mut cursor = Cursor::new([
        1, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
    ]);
    let wkey = cursor.from_bytes::<Kdf>().unwrap();

    assert_eq!(
        wkey,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3]
        }
    );
}

#[test]
fn to_bytes_nospace() {
    let kdf = Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 65536,
        salt: vec![1, 2, 3],
    };
    let mut buf = [0; 13];

    for i in 0..buf.len() {
        let mut cursor = Cursor::new(&mut buf[..i]);
        let err = cursor.to_bytes(&kdf).unwrap_err();
        assert_error!(err, BytesError::NoSpace);
    }
}

#[test]
fn to_bytes_none() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&Kdf::None).unwrap();

    assert_eq!(cursor.into_inner(), [0]);
}

#[test]
fn to_bytes_pbkdf2() {
    let mut cursor = Cursor::new(vec![]);

    cursor
        .to_bytes(&Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3],
        })
        .unwrap();

    assert_eq!(
        cursor.into_inner(),
        [1, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3]
    );
}
