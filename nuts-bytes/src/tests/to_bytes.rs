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

use crate::ToBytesExt;

#[test]
fn u8_nospace() {
    let mut buf = [1];
    let mut cursor = Cursor::new(&mut buf[..0]);

    let err = cursor.to_bytes(&1u8).unwrap_err();
    assert_eq!(format!("{:?}", err), "NoSpace");
}

#[test]
fn u8() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&0x01u8).unwrap();

    assert_eq!(cursor.into_inner(), [1]);
}

#[test]
fn u16_nospace() {
    let mut buf = [1, 2];

    for i in 0..buf.len() {
        let mut cursor = Cursor::new(&mut buf[..i]);

        let err = cursor.to_bytes(&1u16).unwrap_err();
        assert_eq!(format!("{:?}", err), "NoSpace");
    }
}

#[test]
fn u16() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&0x0102u16).unwrap();

    assert_eq!(cursor.into_inner(), [1, 2]);
}

#[test]
fn u32_nospace() {
    let mut buf = [1, 2, 3];

    for i in 0..buf.len() {
        let mut cursor = Cursor::new(&mut buf[..i]);

        let err = cursor.to_bytes(&0x010203u32).unwrap_err();
        assert_eq!(format!("{:?}", err), "NoSpace");
    }
}

#[test]
fn u32() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&0x01020304u32).unwrap();

    assert_eq!(cursor.into_inner(), [1, 2, 3, 4]);
}

#[test]
fn u64_nospace() {
    let mut buf = [1, 2, 3, 4, 5, 6, 7];

    for i in 0..buf.len() {
        let mut cursor = Cursor::new(&mut buf[..i]);

        let err = cursor.to_bytes(&0x0102030405060708u64).unwrap_err();
        assert_eq!(format!("{:?}", err), "NoSpace");
    }
}

#[test]
fn u64() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&0x0102030405060708u64).unwrap();

    assert_eq!(cursor.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn slice_empty() {
    let buf: [u8; 0] = [];
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&&buf[..]).unwrap();

    assert_eq!(cursor.into_inner(), [0, 0, 0, 0]);
}

#[test]
fn slice() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&&[1u8, 2u8, 3u8][..]).unwrap();

    assert_eq!(cursor.into_inner(), [0, 0, 0, 3, 1, 2, 3]);
}

#[test]
fn slice_nospace() {
    let mut buf = [0, 0, 0, 3, 1, 2, 3];

    for i in 0..buf.len() {
        let mut cursor = Cursor::new(&mut buf[..i]);

        let err = cursor.to_bytes(&&[1u8, 2u8, 3u8][..]).unwrap_err();
        assert_eq!(format!("{:?}", err), "NoSpace");
    }
}
