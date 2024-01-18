// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use crate::{PutBytes, PutBytesError};

#[test]
fn empty_slice_put_zero() {
    let mut buf = [0; 0];

    buf.as_mut().put_bytes(&[]).unwrap();
}

#[test]
fn empty_slice_put_one() {
    let mut buf = [0; 0];

    let err = buf.as_mut().put_bytes(&[1]).unwrap_err();
    assert_eq!(err, PutBytesError::NoSpace);
}

#[test]
fn slice_put_zero() {
    let mut buf = [b'x'; 3];

    buf.as_mut().put_bytes(&[]).unwrap();
    assert_eq!(buf, [b'x', b'x', b'x']);
}

#[test]
fn slice_put_one() {
    let mut buf = [b'x'; 3];

    buf.as_mut().put_bytes(&[1]).unwrap();
    assert_eq!(buf, [1, b'x', b'x']);
}

#[test]
fn slice_put_two() {
    let mut buf = [b'x'; 3];

    buf.as_mut().put_bytes(&[1, 2]).unwrap();
    assert_eq!(buf, [1, 2, b'x']);
}

#[test]
fn slice_put_three() {
    let mut buf = [b'x'; 3];

    buf.as_mut().put_bytes(&[1, 2, 3]).unwrap();
    assert_eq!(buf, [1, 2, 3]);
}

#[test]
fn slice_put_four() {
    let mut buf = [b'x'; 3];

    let err = buf.as_mut().put_bytes(&[1, 2, 3, 4]).unwrap_err();
    assert_eq!(err, PutBytesError::NoSpace);
    assert_eq!(buf, [b'x', b'x', b'x']);
}
