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

use std::io::Cursor;

use nuts_bytes::{Error as BytesError, FromBytesExt, ToBytesExt};

use crate::container::cipher::Cipher;

#[test]
fn from_bytes_none() {
    let mut cursor = Cursor::new([0]);
    let cipher = cursor.from_bytes::<Cipher>().unwrap();

    assert_eq!(cipher, Cipher::None);
}

#[test]
fn from_bytes_aes128_ctr() {
    let mut cursor = Cursor::new([1]);
    let cipher = cursor.from_bytes::<Cipher>().unwrap();

    assert_eq!(cipher, Cipher::Aes128Ctr);
}

#[test]
fn from_bytes_invalid() {
    let mut cursor = Cursor::new([2]);

    let err = cursor.from_bytes::<Cipher>().unwrap_err();
    let msg = into_error!(err, BytesError::Invalid);
    assert_eq!(msg, "invalid cipher: 2");
}

#[test]
fn to_bytes_none() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&Cipher::None).unwrap();

    assert_eq!(cursor.into_inner(), [0]);
}

#[test]
fn to_bytes_aes128_ctr() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&Cipher::Aes128Ctr).unwrap();

    assert_eq!(cursor.into_inner(), [1]);
}

#[test]
fn to_bytes_nospace() {
    let mut buf = [];
    let mut cursor = Cursor::new(&mut buf[..]);

    let err = cursor.to_bytes(&Cipher::None).unwrap_err();
    assert_error!(err, BytesError::NoSpace);
}

#[test]
fn block_size_none() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn block_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn key_len_none() {
    assert_eq!(Cipher::None.key_len(), 0);
}

#[test]
fn key_len_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.key_len(), 16);
}

#[test]
fn iv_len_none() {
    assert_eq!(Cipher::None.iv_len(), 0);
}

#[test]
fn iv_len_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.iv_len(), 16);
}
