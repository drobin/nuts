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
use crate::openssl::HMAC;
use crate::types::Digest;

#[test]
fn create_sha1() {
    let hmac = HMAC::create(Digest::Sha1, b"123", b"abc").unwrap();
    assert_eq!(hmac.len(), 20);
}

#[test]
fn verifiy_ok_sha1() {
    let hmac = HMAC::create(Digest::Sha1, b"123", b"abc").unwrap();
    HMAC::verify(Digest::Sha1, b"123", b"abc", &hmac).unwrap();
}

#[test]
fn verifiy_wrong_key_sha1() {
    let hmac = HMAC::create(Digest::Sha1, b"123", b"abc").unwrap();
    let err = HMAC::verify(Digest::Sha1, b"1234", b"abc", &hmac).unwrap_err();

    assert_eq!(format!("{:?}", err), "HmacMismatch");
}

#[test]
fn verifiy_wrong_data_sha1() {
    let hmac = HMAC::create(Digest::Sha1, b"123", b"abc").unwrap();
    let err = HMAC::verify(Digest::Sha1, b"123", b"Xbc", &hmac).unwrap_err();

    assert_eq!(format!("{:?}", err), "HmacMismatch");
}

#[test]
fn verifiy_wrong_hmac_sha1() {
    let mut hmac = HMAC::create(Digest::Sha1, b"123", b"abc").unwrap();

    let elem = hmac.get_mut(0).unwrap();
    *elem += 1;

    let err = HMAC::verify(Digest::Sha1, b"123", b"abc", &hmac).unwrap_err();

    assert_eq!(format!("{:?}", err), "HmacMismatch");
}
