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

use crate::FromBytesExt;

#[test]
fn u8_eof() {
    let mut cursor = Cursor::new([]);
    let err = cursor.from_bytes::<u8>().unwrap_err();

    assert_eq!(format!("{:?}", err), "Eof");
}

#[test]
fn u8() {
    let mut cursor = Cursor::new([1]);
    let n = cursor.from_bytes::<u8>().unwrap();

    assert_eq!(n, 0x01);
}

#[test]
fn u16_eof() {
    const BUF: [u8; 1] = [1];

    for i in 0..BUF.len() {
        let mut cursor = Cursor::new(&BUF[..i]);
        let err = cursor.from_bytes::<u16>().unwrap_err();

        assert_eq!(format!("{:?}", err), "Eof");
    }
}

#[test]
fn u16() {
    let mut cursor = Cursor::new([1, 2]);
    let n = cursor.from_bytes::<u16>().unwrap();

    assert_eq!(n, 0x0102);
}

#[test]
fn u32_eof() {
    const BUF: [u8; 3] = [1, 2, 3];

    for i in 0..BUF.len() {
        let mut cursor = Cursor::new(&BUF[..i]);
        let err = cursor.from_bytes::<u32>().unwrap_err();

        assert_eq!(format!("{:?}", err), "Eof");
    }
}

#[test]
fn u32() {
    let mut cursor = Cursor::new([1, 2, 3, 4]);
    let n = cursor.from_bytes::<u32>().unwrap();

    assert_eq!(n, 0x01020304);
}

#[test]
fn u64_eof() {
    const BUF: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];

    for i in 0..BUF.len() {
        let mut cursor = Cursor::new(&BUF[..i]);
        let err = cursor.from_bytes::<u64>().unwrap_err();

        assert_eq!(format!("{:?}", err), "Eof");
    }
}

#[test]
fn u64() {
    let mut cursor = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);
    let n = cursor.from_bytes::<u64>().unwrap();

    assert_eq!(n, 0x0102030405060708);
}

#[test]
fn vec_empty() {
    let mut cursor = Cursor::new([0, 0, 0, 0]);
    let vec = cursor.from_bytes::<Vec<u8>>().unwrap();

    assert!(vec.is_empty());
}

#[test]
fn vec() {
    let mut cursor = Cursor::new([0, 0, 0, 3, 1, 2, 3]);
    let vec = cursor.from_bytes::<Vec<u8>>().unwrap();

    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn vec_eof() {
    const BUF: [u8; 7] = [0, 0, 0, 3, 1, 2, 3];

    for i in 0..BUF.len() {
        let mut cursor = Cursor::new(&BUF[..i]);
        let err = cursor.from_bytes::<Vec<u8>>().unwrap_err();

        assert_eq!(format!("{:?}", err), "Eof");
    }
}
