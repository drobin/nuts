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

use crate::cipher::Cipher;
use crate::header::inner::{Inner, Revision};
use crate::header::secret::Secret;
use crate::header::{rev0, HeaderError};
use crate::kdf::Kdf;

#[test]
fn new() {
    let rev0 = rev0::Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: Secret::new(vec![]),
    };
    let inner = Inner::new(Revision::Rev0(rev0));

    assert_eq!(inner.magic, *b"nuts-io");
}

#[test]
fn de_inval_magic() {
    let buf = *b"xuts-io";
    let err = Inner::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, HeaderError::InvalidHeader));
}

#[test]
fn de_rev0() {
    let buf = [
        b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
        0x00, 0x00, 0x00, 0x00, // rev0 variant,
        0x00, 0x00, 0x00, 0x00, // cipher
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
        0x00, 0x00, 0x00, 0x00, // kdf
        0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
    ];

    let inner = Inner::get_from_buffer(&mut &buf[..]).unwrap();
    let Revision::Rev0(rev0) = inner.rev;

    assert_eq!(rev0.cipher, Cipher::None);
    assert_eq!(rev0.iv, []);
    assert_eq!(rev0.kdf, Kdf::None);
    assert_eq!(rev0.secret, [1, 2, 3]);
}

#[test]
fn ser_rev0() {
    let mut buf = vec![];
    let inner = Inner::new(Revision::Rev0(rev0::Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: Secret::new(vec![1, 2, 3]),
    }));

    inner.put_into_buffer(&mut buf).unwrap();
    assert_eq!(
        buf,
        [
            b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
            0x00, 0x00, 0x00, 0x00, // rev0 variant,
            0x00, 0x00, 0x00, 0x00, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
            0x00, 0x00, 0x00, 0x00, // kdf
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
        ]
    );
}
