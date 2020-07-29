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
use crate::openssl::pbkdf2;
use crate::types::Digest;

#[test]
fn digest_some() {
    let wkey = pbkdf2(b"123", b"123", 1, Digest::Sha1).unwrap();
    assert_eq!(
        wkey,
        [58, 68, 159, 34, 207, 49, 175, 62, 2, 158, 184, 166, 204, 23, 216, 35, 160, 87, 69, 60]
    );
}

#[test]
fn empty_password() {
    if let Error::InvalArg(msg) = pbkdf2(b"", b"123", 1, Digest::Sha1).unwrap_err() {
        assert_eq!(msg, "invalid password, cannot be empty");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn empty_salt() {
    if let Error::InvalArg(msg) = pbkdf2(b"123", b"", 1, Digest::Sha1).unwrap_err() {
        assert_eq!(msg, "invalid salt, cannot be empty");
    } else {
        panic!("invalid error");
    }
}
