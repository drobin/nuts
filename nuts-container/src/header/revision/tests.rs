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

use crate::buffer::BufferError;
use crate::cipher::Cipher;
use crate::header::revision::{Data, Revision};
use crate::header::HeaderError;
use crate::kdf::Kdf;
use crate::tests::into_error;

const REV0: [u8; 38] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0x00, 0x00, 0x00, 0x00, // revision
    0x00, 0x00, 0x00, 0x00, // cipher
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
    0x00, 0x00, 0x00, 0x00, // kdf
    0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
];

const REV1: [u8; 38] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0x00, 0x00, 0x00, 0x01, // revision
    0x00, 0x00, 0x00, 0x00, // cipher
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
    0x00, 0x00, 0x00, 0x00, // kdf
    0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
];

#[test]
fn new_rev0() {
    let revision = Revision::new_rev0(Cipher::None, vec![1], Kdf::None, vec![2, 3]);

    let expected = Data {
        cipher: Cipher::None,
        iv: vec![1],
        kdf: Kdf::None,
        secret: vec![2, 3],
    };

    assert!(matches!(revision, Revision::Rev0(data) if data == expected));
}

#[test]
fn new_rev1() {
    let revision = Revision::new_rev1(Cipher::None, vec![1], Kdf::None, vec![2, 3]);

    let expected = Data {
        cipher: Cipher::None,
        iv: vec![1],
        kdf: Kdf::None,
        secret: vec![2, 3],
    };

    assert!(matches!(revision, Revision::Rev1(data) if data == expected));
}

#[test]
fn de_inval_revision() {
    let mut buf = REV0;

    buf[10] = 2;

    let err = Revision::get_from_buffer(&mut &buf[..]).unwrap_err();
    let err = into_error!(err, HeaderError::Buffer);
    let (str, idx) = into_error!(err, BufferError::InvalidIndex(2));

    assert_eq!(str, "Revision");
    assert_eq!(idx, 2);
}

#[test]
fn de_rev0() {
    match Revision::get_from_buffer(&mut &REV0[..]).unwrap() {
        Revision::Rev0(rev0) => {
            assert_eq!(rev0.cipher, Cipher::None);
            assert_eq!(rev0.iv, []);
            assert_eq!(rev0.kdf, Kdf::None);
            assert_eq!(rev0.secret, [1, 2, 3]);
        }
        Revision::Rev1(_) => panic!("invalid revision"),
    }
}

#[test]
fn de_rev0_inval_magic() {
    let mut buf = REV0;

    buf[0] = b'x';

    let err = Revision::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, HeaderError::InvalidHeader));
}

#[test]
fn ser_rev0() {
    let mut buf = vec![];
    let inner = Revision::Rev0(Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: vec![1, 2, 3],
    });

    inner.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV0);
}

#[test]
fn de_rev1() {
    match Revision::get_from_buffer(&mut &REV1[..]).unwrap() {
        Revision::Rev0(_) => panic!("invalid revision"),
        Revision::Rev1(rev1) => {
            assert_eq!(rev1.cipher, Cipher::None);
            assert_eq!(rev1.iv, []);
            assert_eq!(rev1.kdf, Kdf::None);
            assert_eq!(rev1.secret, [1, 2, 3]);
        }
    }
}

#[test]
fn de_rev1_inval_magic() {
    let mut buf = REV1;

    buf[0] = b'x';

    let err = Revision::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, HeaderError::InvalidHeader));
}

#[test]
fn ser_rev1() {
    let mut buf = vec![];
    let inner = Revision::Rev1(Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: vec![1, 2, 3],
    });

    inner.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV1);
}
