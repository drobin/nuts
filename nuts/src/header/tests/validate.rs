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

use crate::header::Header;
use crate::types::{Cipher, Digest};
use crate::wkey::{Pbkdf2Data, WrappingKeyData};

fn ok_header() -> Header {
    Header {
        revision: 1,
        cipher: Cipher::Aes128Ctr,
        digest: Some(Digest::Sha1),
        wrapping_key: Some(WrappingKeyData::Pbkdf2(Pbkdf2Data {
            iterations: 4711,
            salt: vec![1, 2, 3],
        })),
        wrapping_iv: vec![
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        ],
        hmac: vec![
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ],
        secret: vec![8, 9, 10, 11, 12],
    }
}

#[test]
fn bad_revision() {
    let mut header = ok_header();
    header.revision = 0;

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalRevision)");
}

#[test]
fn cipher_none_digest_some() {
    let mut header = ok_header();
    header.cipher = Cipher::None;
    header.digest = Some(Digest::Sha1);

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDigest)");
}

#[test]
fn cipher_some_digest_none() {
    let mut header = ok_header();
    header.cipher = Cipher::Aes128Ctr;
    header.digest = None;

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDigest)");
}

#[test]
fn cipher_none_empty_hmac_iv_accepted() {
    let mut header = ok_header();
    header.cipher = Cipher::None;
    header.digest = None;
    header.wrapping_iv.clear();
    header.hmac.clear();

    header.validate().unwrap();
}

#[test]
fn cipher_none_hmac_rejected() {
    let mut header = ok_header();
    header.cipher = Cipher::None;
    header.digest = None;
    header.wrapping_iv.clear();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalHmac)");
}

#[test]
fn cipher_none_iv_rejected() {
    let mut header = ok_header();
    header.cipher = Cipher::None;
    header.digest = None;
    header.hmac.clear();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalIv)");
}

#[test]
fn cipher_some_empty_hmac() {
    let mut header = ok_header();
    header.hmac.clear();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalHmac)");
}

#[test]
fn cipher_some_hmac_inval_size() {
    let mut header = ok_header();
    header.hmac.pop();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalHmac)");
}

#[test]
fn cipher_some_empty_wrapping_iv() {
    let mut header = ok_header();
    header.wrapping_iv.clear();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalIv)");
}

#[test]
fn cipher_some_wrapping_iv_inval_size() {
    let mut header = ok_header();
    header.wrapping_iv.pop();

    let err = header.validate().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalIv)");
}
