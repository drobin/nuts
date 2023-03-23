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

use nuts_bytes::bytes::Error;

use crate::container::cipher::Cipher;
use crate::container::header::inner::{Inner, Revision};
use crate::container::header::secret::Secret;
use crate::container::header::{bytes_options, rev0};
use crate::container::kdf::Kdf;

#[test]
fn new() {
    let rev0 = rev0::Data {
        cipher: Cipher::None,
        iv: vec![],
        kdf: Kdf::None,
        secret: Secret::new(vec![]),
    };
    let inner = Inner::new(Revision::Rev0(rev0));

    assert_eq!(inner.magic, b"nuts-io");
}

#[test]
fn de_inval_magic() {
    let err = bytes_options().from_bytes::<Inner>(b"xuts-io").unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(msg, "invalid magic");
}

#[test]
fn de_rev0() {
    let inner = bytes_options()
        .from_bytes::<Inner>(&[
            b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
            0,    // rev0 variant,
            0,    // cipher
            0,    // iv,
            0,    // kdf
            3, 1, 2, 3, // secret
        ])
        .unwrap();

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

    let vec = bytes_options().to_vec(&inner).unwrap();
    assert_eq!(
        vec,
        [
            b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
            0,    // rev0 variant,
            0,    // cipher
            0,    // iv,
            0,    // kdf
            3, 1, 2, 3, // secret
        ]
    );
}
