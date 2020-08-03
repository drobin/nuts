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

use crate::error::Error;
use crate::types::Digest;

#[test]
fn from_string_none() {
    assert_eq!(Digest::from_string("none").unwrap(), None);
}

#[test]
fn from_string_sha1() {
    assert_eq!(Digest::from_string("sha1").unwrap(), Some(Digest::Sha1));
}

#[test]
fn from_string_inval() {
    let err = Digest::from_string("xxx").unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "invalid digest: xxx");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn size_sha1() {
    assert_eq!(Digest::Sha1.size(), 20);
}

#[test]
fn display_sha1() {
    assert_eq!(format!("{}", Digest::Sha1), "sha1");
}
