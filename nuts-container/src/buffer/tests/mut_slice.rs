// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use crate::buffer::{BufferError, BufferMut};

#[test]
fn put_chunk_empty_none() {
    let mut slice = [].as_mut_slice();

    slice.put_chunk(&[]).unwrap();

    assert!(slice.is_empty());
}

#[test]
fn put_chunk_empty_some() {
    let mut slice = [].as_mut_slice();
    let err = slice.put_chunk(&[1, 2, 3]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
}

#[test]
fn put_chunk_none() {
    let mut buf = [b'x'; 3];

    buf.as_mut_slice().put_chunk(&[]).unwrap();

    assert_eq!(&buf, b"xxx");
}

#[test]
fn put_chunk_some() {
    let mut buf = [b'x'; 3];

    buf.as_mut_slice().put_chunk(&[1]).unwrap();

    assert_eq!(buf, [1, b'x', b'x']);
}

#[test]
fn put_chunk_all() {
    let mut buf = [b'x'; 3];

    buf.as_mut_slice().put_chunk(&[1, 2, 3]).unwrap();

    assert_eq!(buf, [1, 2, 3]);
}

#[test]
fn put_chunk_more() {
    let mut buf = [b'x'; 3];
    let err = buf.as_mut_slice().put_chunk(&[1, 2, 3, 4]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"xxx");
}

#[test]
fn put_u8() {
    let mut buf = [b'x'; 2];

    buf.as_mut_slice().put_u8(1).unwrap();

    assert_eq!(buf, [1, b'x']);
}

#[test]
fn put_u8_eof() {
    let mut buf = [];
    let err = buf.as_mut_slice().put_u8(1).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
}

#[test]
fn put_u16() {
    let mut buf = [b'x'; 3];

    buf.as_mut_slice().put_u16(258).unwrap();

    assert_eq!(buf, [1, 2, b'x']);
}

#[test]
fn put_u16_eof() {
    let mut buf = [b'x'; 1];
    let err = buf.as_mut_slice().put_u16(258).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"x");
}

#[test]
fn put_u32() {
    let mut buf = [b'x'; 5];

    buf.as_mut_slice().put_u32(16_909_060).unwrap();

    assert_eq!(buf, [1, 2, 3, 4, b'x']);
}

#[test]
fn put_u32_eof() {
    let mut buf = [b'x'; 3];
    let err = buf.as_mut_slice().put_u32(16_909_060).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"xxx");
}

#[test]
fn put_u64() {
    let mut buf = [b'x'; 9];

    buf.as_mut_slice().put_u64(72_623_859_790_382_856).unwrap();

    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8, b'x']);
}

#[test]
fn put_u64_eof() {
    let mut buf = [b'x'; 7];
    let err = buf
        .as_mut_slice()
        .put_u64(72_623_859_790_382_856)
        .unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"xxxxxxx");
}

#[test]
fn put_usize() {
    let mut buf = [b'x'; 9];

    buf.as_mut_slice()
        .put_usize(72_623_859_790_382_856)
        .unwrap();

    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8, b'x']);
}

#[test]
fn put_usize_eof() {
    let mut buf = [b'x'; 7];
    let err = buf
        .as_mut_slice()
        .put_usize(72_623_859_790_382_856)
        .unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"xxxxxxx");
}

#[test]
fn put_vec_len_eof() {
    let mut buf = [b'x'; 7];
    let err = buf.as_mut_slice().put_vec(&[]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(&buf, b"xxxxxxx");
}

#[test]
fn put_vec_data_eof() {
    let mut buf = [b'x'; 10];
    let err = buf.as_mut_slice().put_vec(&[1, 2, 3]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, b'x', b'x']);
}

#[test]
fn put_vec_empty() {
    let mut buf = [b'x'; 9];

    buf.as_mut_slice().put_vec(&[]).unwrap();

    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 0, b'x']);
}

#[test]
fn put_vec() {
    let mut buf = [b'x'; 12];

    buf.as_mut_slice().put_vec(&[1, 2, 3]).unwrap();

    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 3, b'x']);
}
