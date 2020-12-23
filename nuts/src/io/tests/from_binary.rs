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

use crate::io::FromBinary;

#[test]
fn from_u8() {
    let mut c = Cursor::new([1, 2]);

    assert_eq!(u8::from_binary(&mut c).unwrap(), 1);
    assert_eq!(c.position(), 1);

    assert_eq!(u8::from_binary(&mut c).unwrap(), 2);
    assert_eq!(c.position(), 2);

    let err = u8::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 2);
}

#[test]
fn from_u16() {
    let mut c = Cursor::new([1, 2, 3, 4]);

    assert_eq!(u16::from_binary(&mut c).unwrap(), 0x0102);
    assert_eq!(c.position(), 2);

    assert_eq!(u16::from_binary(&mut c).unwrap(), 0x0304);
    assert_eq!(c.position(), 4);

    let err = u16::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 4);
}

#[test]
fn from_u16_not_enough_data() {
    let mut c = Cursor::new([1, 2, 3]);

    assert_eq!(u16::from_binary(&mut c).unwrap(), 0x0102);
    assert_eq!(c.position(), 2);

    let err = u16::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 2);
}

#[test]
fn from_u32() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(u32::from_binary(&mut c).unwrap(), 0x01020304);
    assert_eq!(c.position(), 4);

    assert_eq!(u32::from_binary(&mut c).unwrap(), 0x05060708);
    assert_eq!(c.position(), 8);

    let err = u32::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 8);
}

#[test]
fn from_u32_not_enough_data() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(u32::from_binary(&mut c).unwrap(), 0x01020304);
    assert_eq!(c.position(), 4);

    let err = u32::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 4);
}

#[test]
fn from_u64() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

    assert_eq!(u64::from_binary(&mut c).unwrap(), 0x0102030405060708);
    assert_eq!(c.position(), 8);

    assert_eq!(u64::from_binary(&mut c).unwrap(), 0x090A0B0C0D0E0F10);
    assert_eq!(c.position(), 16);

    let err = u64::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 16);
}

#[test]
fn from_u64_not_enough_data() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    assert_eq!(u64::from_binary(&mut c).unwrap(), 0x0102030405060708);
    assert_eq!(c.position(), 8);

    let err = u64::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 8);
}

#[test]
fn from_usize() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(usize::from_binary(&mut c).unwrap(), 0x01020304);
    assert_eq!(c.position(), 4);

    assert_eq!(usize::from_binary(&mut c).unwrap(), 0x05060708);
    assert_eq!(c.position(), 8);

    let err = usize::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 8);
}

#[test]
fn from_usize_not_enough_data() {
    let mut c = Cursor::new([1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(usize::from_binary(&mut c).unwrap(), 0x01020304);
    assert_eq!(c.position(), 4);

    let err = usize::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 4);
}

#[test]
fn from_vec_empty() {
    let mut c = Cursor::new([0, 0, 0, 0]);

    let vec = Vec::<u8>::from_binary(&mut c).unwrap();
    assert_eq!(vec.len(), 0);
    assert_eq!(c.position(), 4);
}

#[test]
fn from_vec_not_empty() {
    let mut c = Cursor::new([0, 0, 0, 2, 3, 4]);

    let vec = Vec::<u8>::from_binary(&mut c).unwrap();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [3, 4]);
    assert_eq!(c.position(), 6);
}

#[test]
fn from_vec_not_enough_data_for_size() {
    let mut c = Cursor::new([0, 0, 0]);

    let err = Vec::<u8>::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 0);
}

#[test]
fn from_vec_not_empty_not_enough_data_for_elem_1() {
    let mut c = Cursor::new([0, 0, 0, 2]);

    let err = Vec::<u8>::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 4);
}

#[test]
fn from_vec_not_empty_not_enough_data_for_elem_2() {
    let mut c = Cursor::new([0, 0, 0, 2, 1]);

    let err = Vec::<u8>::from_binary(&mut c).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    assert_eq!(c.position(), 5);
}
