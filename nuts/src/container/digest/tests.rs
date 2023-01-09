// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use crate::bytes::{Error as BytesError, FromBytesExt, ToBytesExt};
use crate::container::Digest;

#[test]
fn size_sha1() {
    assert_eq!(Digest::Sha1.size(), 20);
}

#[test]
fn fromn_bytes_sha1() {
    let mut cursor = Cursor::new([1]);

    assert_eq!(cursor.from_bytes::<Digest>().unwrap(), Digest::Sha1);
}

#[test]
fn from_bytes_eof() {
    let mut cursor = Cursor::new([]);
    let err = cursor.from_bytes::<Digest>().unwrap_err();

    assert_error!(err, BytesError::Eof);
}

#[test]
fn from_bytes_inval() {
    let mut cursor = Cursor::new([2]);
    let err = cursor.from_bytes::<Digest>().unwrap_err();

    let msg = into_error!(err, BytesError::Invalid);
    assert_eq!(msg, "invalid digest: 2");
}

#[test]
fn to_bytes_nospace() {
    let mut buf = [];
    let mut cursor = Cursor::new(&mut buf[..]);

    let err = cursor.to_bytes(&Digest::Sha1).unwrap_err();
    assert_error!(err, BytesError::NoSpace);
}

#[test]
fn to_bytes_sha1() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&Digest::Sha1).unwrap();

    assert_eq!(cursor.into_inner(), [1]);
}
