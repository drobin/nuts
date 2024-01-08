// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use crate::{take_bytes::TakeBytes, TakeBytesError};

#[test]
fn empty_slice_zero() {
    [0; 0].as_slice().take_bytes(&mut []).unwrap();
}

#[test]
fn empty_slice_one() {
    let mut buf = [b'x'; 1];

    let err = [].as_slice().take_bytes(&mut buf).unwrap_err();
    assert_eq!(err, TakeBytesError::Eof);
    assert_eq!(buf, [b'x']);
}

#[test]
fn slice_zero() {
    let mut slice = [1, 2, 3].as_slice();

    slice.take_bytes(&mut []).unwrap();
    assert_eq!(slice, [1, 2, 3]);
}

#[test]
fn slice_one() {
    let mut slice = [1, 2, 3].as_slice();
    let mut buf = [b'x'; 1];

    slice.take_bytes(&mut buf).unwrap();
    assert_eq!(buf, [1]);
    assert_eq!(slice, [2, 3]);
}

#[test]
fn slice_two() {
    let mut slice = [1, 2, 3].as_slice();
    let mut buf = [b'x'; 2];

    slice.take_bytes(&mut buf).unwrap();
    assert_eq!(buf, [1, 2]);
    assert_eq!(slice, [3]);
}

#[test]
fn slice_three() {
    let mut slice = [1, 2, 3].as_slice();
    let mut buf = [b'x'; 3];

    slice.take_bytes(&mut buf).unwrap();
    assert_eq!(buf, [1, 2, 3]);
    assert_eq!(slice, []);
}

#[test]
fn slice_four() {
    let mut slice = [1, 2, 3].as_slice();
    let mut buf = [b'x'; 4];

    let err = slice.take_bytes(&mut buf).unwrap_err();
    assert_eq!(err, TakeBytesError::Eof);
    assert_eq!(buf, [b'x', b'x', b'x', b'x']);
    assert_eq!(slice, [1, 2, 3]);
}
