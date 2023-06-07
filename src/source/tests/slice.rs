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

use std::borrow::Cow;

use crate::assert_error;
use crate::error::Error;
use crate::source::TakeBytes;

#[test]
fn take_bytes() {
    let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice();

    assert_eq!(source.take_bytes(0).unwrap(), Cow::Borrowed(&[]));
    assert_eq!(source, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(source.take_bytes(1).unwrap(), Cow::Borrowed(&[1]));
    assert_eq!(source, [2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(source.take_bytes(2).unwrap(), Cow::Borrowed(&[2, 3]));
    assert_eq!(source, [4, 5, 6, 7, 8, 9]);
    assert_eq!(source.take_bytes(3).unwrap(), Cow::Borrowed(&[4, 5, 6]));
    assert_eq!(source, [7, 8, 9]);

    let err = source.take_bytes(4).unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(source, [7, 8, 9]);
}

#[test]
fn take_bytes_to() {
    let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice();

    let mut buf = [];
    source.take_bytes_to(&mut buf).unwrap();
    assert_eq!(source, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 1];
    source.take_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [1]);
    assert_eq!(source, [2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 2];
    source.take_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [2, 3]);
    assert_eq!(source, [4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 3];
    source.take_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [4, 5, 6]);
    assert_eq!(source, [7, 8, 9]);

    let mut buf = [0; 4];
    let err = source.take_bytes_to(&mut buf).unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(buf, [0, 0, 0, 0]);
    assert_eq!(source, [7, 8, 9]);
}
