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

use nuts_bytes::{Error, Reader, Writer};

use crate::container::cipher::Cipher;
use crate::container::header::inner::{Inner, Revision};
use crate::container::header::rev0;
use crate::container::header::secret::Secret;
use crate::container::header::HeaderMagicError;
use crate::container::kdf::Kdf;
use crate::tests::into_error;

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
    let mut reader = Reader::new(b"xuts-io".as_slice());
    let err = reader.read::<Inner>().unwrap_err();
    let err = into_error!(err, Error::Custom);
    assert!(err.is::<HeaderMagicError>());
}

#[test]
fn de_rev0() {
    let mut reader = Reader::new(
        [
            b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
            0x00, 0x00, 0x00, 0x00, // rev0 variant,
            0x00, 0x00, 0x00, 0x00, // cipher
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // iv,
            0x00, 0x00, 0x00, 0x00, // kdf
            0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // secret
        ]
        .as_slice(),
    );
    let inner = reader.read::<Inner>().unwrap();

    let Revision::Rev0(rev0) = inner.rev;

    assert_eq!(rev0.cipher, Cipher::None);
    assert_eq!(rev0.iv, []);
    assert_eq!(rev0.kdf, Kdf::None);
    assert_eq!(rev0.secret, [1, 2, 3]);
}

#[test]
fn ser_rev0() {
    let inner = Inner::new(Revision::Rev0(rev0::Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: Secret::new(vec![1, 2, 3]),
    }));

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&inner).unwrap(), 38);
    assert_eq!(
        writer.into_target(),
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
