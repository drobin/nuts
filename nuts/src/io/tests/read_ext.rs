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

use crate::io::ReadExt;

fn read_remaining(c: &mut Cursor<Vec<u8>>) -> Vec<u8> {
    let mut v = vec![];
    c.read_to_end(&mut v).unwrap();
    v
}

#[test]
fn array_empty_complete() {
    let mut c = Cursor::new(vec![]);

    let arr = c.read_array(0).unwrap();
    assert_eq!(arr.len(), 0);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn array_empty_remaining() {
    let mut c = Cursor::new(vec![b'x']);

    let arr = c.read_array(0).unwrap();
    assert_eq!(arr.len(), 0);
    assert_eq!(read_remaining(&mut c), [b'x'])
}

#[test]
fn array_non_empty_no_data() {
    for i in 0..2 {
        let mut c = Cursor::new(vec![1, 2, 3]);
        c.get_mut().resize(i, 0);

        let err = c.read_array(3).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }
}

#[test]
fn array_non_empty_complete() {
    let mut c = Cursor::new(vec![1, 2, 3]);

    assert_eq!(c.read_array(3).unwrap(), [1, 2, 3]);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn array_non_empty_remaining() {
    let mut c = Cursor::new(vec![1, 2, 3, b'x']);

    assert_eq!(c.read_array(3).unwrap(), [1, 2, 3]);
    assert_eq!(read_remaining(&mut c), [b'x']);
}

#[test]
fn vec_empty_no_data() {
    for i in 0..4 {
        let mut c = Cursor::new(vec![0, 0, 0, 0]);
        c.get_mut().resize(i, 0);

        let err = c.read_vec().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }
}

#[test]
fn vec_empty_complete() {
    let mut c = Cursor::new(vec![0, 0, 0, 0]);

    assert_eq!(c.read_vec().unwrap(), []);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn vec_empty_remaining() {
    let mut c = Cursor::new(vec![0, 0, 0, 0, b'x']);

    assert_eq!(c.read_vec().unwrap(), []);
    assert_eq!(read_remaining(&mut c), [b'x']);
}

#[test]
fn vec_non_empty_no_data() {
    for i in 0..7 {
        let mut c = Cursor::new(vec![0, 0, 0, 3, b'a', b'b', b'c']);
        c.get_mut().resize(i, 0);

        let err = c.read_vec().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }
}

#[test]
fn vec_non_empty_complete() {
    let mut c = Cursor::new(vec![0, 0, 0, 3, b'a', b'b', b'c']);

    assert_eq!(c.read_vec().unwrap(), [b'a', b'b', b'c']);
    assert_eq!(read_remaining(&mut c), []);
}

#[test]
fn vec_non_empty_remaining() {
    let mut c = Cursor::new(vec![0, 0, 0, 3, b'a', b'b', b'c', b'x']);

    assert_eq!(c.read_vec().unwrap(), [b'a', b'b', b'c']);
    assert_eq!(read_remaining(&mut c), [b'x']);
}
