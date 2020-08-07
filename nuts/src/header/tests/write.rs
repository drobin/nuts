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

use std::io::ErrorKind;

use crate::error::Error;
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
        iv: vec![13, 14, 15, 16, 17, 18],
        hmac: vec![4, 5, 6, 7],
        secret: vec![8, 9, 10, 11, 12],
    }
}

fn setup() -> (Header, [u8; 64]) {
    (ok_header(), [0; 64])
}

#[test]
fn ok() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 1); // cipher
    assert_eq!(target[9], 1); // digest
    assert_eq!(
        target[10..22],
        [1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3]
    ); // pbkdf2
    assert_eq!(
        target[22..32],
        [0x00, 0x00, 0x00, 0x06, 13, 14, 15, 16, 17, 18]
    ); // iv
    assert_eq!(target[32..40], [0x00, 0x00, 0x00, 0x04, 4, 5, 6, 7]); // hmac
    assert_eq!(target[40..49], [0x00, 0x00, 0x00, 0x05, 8, 9, 10, 11, 12]); // secret
}

#[test]
fn no_space() {
    let (header, mut target) = setup();

    for i in 1..49 {
        if let Error::IoError(err) = header.write(&mut target.get_mut(..i).unwrap()).unwrap_err() {
            assert_eq!(err.kind(), ErrorKind::WriteZero);
        } else {
            panic!("invalid error");
        }
    }
}

#[test]
fn cipher_none() {
    let (mut header, mut target) = setup();
    header.cipher = Cipher::None;

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[8], 0);
}

#[test]
fn cipher_aes128_ctr() {
    let (mut header, mut target) = setup();
    header.cipher = Cipher::Aes128Ctr;

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[8], 1);
}

#[test]
fn digest_none() {
    let (mut header, mut target) = setup();
    header.digest = None;

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[9], 0xff);
}

#[test]
fn digest_sha1() {
    let (mut header, mut target) = setup();
    header.digest = Some(Digest::Sha1);

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[9], 1);
}

#[test]
fn wrapping_key_none() {
    let (mut header, mut target) = setup();
    header.wrapping_key = None;

    assert_eq!(header.write(&mut target).unwrap(), 38);
    assert_eq!(target[10], 0xff);
}

#[test]
fn wrapping_key_pbkdf2() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(
        target[10..22],
        [1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3]
    );
}

#[test]
fn iv_empty() {
    let (mut header, mut target) = setup();
    header.iv.clear();

    assert_eq!(header.write(&mut target).unwrap(), 43);
    assert_eq!(target[22..26], [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn iv_non_empty() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(
        target[22..32],
        [0x00, 0x00, 0x00, 0x06, 13, 14, 15, 16, 17, 18]
    );
}

#[test]
fn hmac_empty() {
    let (mut header, mut target) = setup();
    header.hmac.clear();

    assert_eq!(header.write(&mut target).unwrap(), 45);
    assert_eq!(target[32..36], [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn hmac_non_empty() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[32..40], [0x00, 0x00, 0x00, 0x04, 4, 5, 6, 7]);
}

#[test]
fn secret_empty() {
    let (mut header, mut target) = setup();
    header.secret.clear();

    assert_eq!(header.write(&mut target).unwrap(), 44);
    assert_eq!(target[40..44], [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn secret_non_empty() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target).unwrap(), 49);
    assert_eq!(target[40..49], [0x00, 0x00, 0x00, 0x05, 8, 9, 10, 11, 12]);
}
