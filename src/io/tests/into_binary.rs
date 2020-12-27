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

use std::io::{Cursor, ErrorKind};

use crate::io::IntoBinary;

#[test]
fn into_u8() {
    let mut c = Cursor::new(Vec::new());

    1u8.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 1);

    2u8.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 2);

    assert_eq!(c.into_inner(), [1, 2]);
}

#[test]
fn into_u16() {
    let mut c = Cursor::new(Vec::new());

    0x0102u16.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 2);

    0x0304u16.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 4);

    assert_eq!(c.into_inner(), [1, 2, 3, 4]);
}

#[test]
fn into_u32() {
    let mut c = Cursor::new(Vec::new());

    0x01020304u32.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 4);

    0x05060708u32.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 8);

    assert_eq!(c.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn into_u64() {
    let mut c = Cursor::new(Vec::new());

    0x0102030405060708u64.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 8);

    0x090A0B0C0D0E0F10u64.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 16);

    assert_eq!(
        c.into_inner(),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    );
}

#[test]
fn into_usize() {
    let mut c = Cursor::new(Vec::new());

    0x01020304usize.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 4);

    0x05060708usize.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 8);

    assert_eq!(c.into_inner(), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn into_usize_not_fitting() {
    let mut c = Cursor::new(Vec::new());

    let err = 0x100000000usize.into_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::WriteZero);
    assert_eq!(
        format!("{}", err),
        format!("at most {} elements are allowed", u32::MAX)
    );
    assert_eq!(c.position(), 0);
}

#[test]
fn into_vec_empty() {
    let mut c = Cursor::new(Vec::new());

    Vec::<u8>::new().into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 4);

    assert_eq!(c.into_inner(), [0, 0, 0, 0]);
}

#[test]
fn into_vec_not_empty() {
    let mut c = Cursor::new(Vec::new());

    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&[3, 4]);

    vec.into_binary(&mut c).unwrap();
    assert_eq!(c.position(), 6);

    assert_eq!(c.into_inner(), [0, 0, 0, 2, 3, 4]);
}
