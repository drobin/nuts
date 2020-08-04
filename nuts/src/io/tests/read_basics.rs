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

use std::io::{Cursor, ErrorKind, Read};

use crate::io::ReadBasics;

fn read_remaining(c: &mut Cursor<Vec<u8>>) -> Vec<u8> {
    let mut v = vec![];
    c.read_to_end(&mut v).unwrap();
    v
}

#[test]
fn u8_no_data() {
    let mut c = Cursor::new(vec![]);

    let err = c.read_u8().unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn u8_complete() {
    let mut c = Cursor::new(vec![6]);

    assert_eq!(c.read_u8().unwrap(), 6);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn u8_remaining() {
    let mut c = Cursor::new(vec![1, 2, 3]);

    assert_eq!(c.read_u8().unwrap(), 1);
    assert_eq!(read_remaining(&mut c), [2, 3]);
}

#[test]
fn u32_no_data() {
    for i in 0..4 {
        let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68]);
        c.get_mut().resize(i, 0);

        let err = c.read_u32().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }
}

#[test]
fn u32_complete() {
    let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68]);

    assert_eq!(c.read_u32().unwrap(), 308_745_064);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn u32_remaining() {
    let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68, b'x']);

    assert_eq!(c.read_u32().unwrap(), 308_745_064);
    assert_eq!(read_remaining(&mut c), [b'x']);
}

#[test]
fn u64_no_data() {
    for i in 0..8 {
        let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70]);
        c.get_mut().resize(i, 0);

        let err = c.read_u64().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }
}

#[test]
fn u64_complete() {
    let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70]);

    assert_eq!(c.read_u64().unwrap(), 1_326_049_953_023_858_032);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn u64_remaining() {
    let mut c = Cursor::new(vec![0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70, b'x']);

    assert_eq!(c.read_u64().unwrap(), 1_326_049_953_023_858_032);
    assert_eq!(read_remaining(&mut c), [b'x']);
}
