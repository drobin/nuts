// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

use std::io::{Cursor, ErrorKind};

use crate::error::Error;
use crate::io::{FromBinary, IntoBinary};
use crate::rand::RND;
use crate::types::{Digest, WrappingKey};

#[test]
fn pbkdf2() {
    let WrappingKey::Pbkdf2 {
        digest,
        iterations,
        salt,
    } = WrappingKey::pbkdf2(Digest::Sha1, 5, &[1, 2, 3]);

    assert_eq!(digest, Digest::Sha1);
    assert_eq!(iterations, 5);
    assert_eq!(salt, [1, 2, 3]);
}

#[test]
fn generate_pbkdf2_empty_salt() {
    let WrappingKey::Pbkdf2 {
        digest,
        iterations,
        salt,
    } = WrappingKey::generate_pbkdf2(Digest::Sha1, 5, 0).unwrap();

    assert_eq!(digest, Digest::Sha1);
    assert_eq!(iterations, 5);
    assert_eq!(salt, []);
}

#[test]
fn generate_pbkdf2_with_salt() {
    let WrappingKey::Pbkdf2 {
        digest,
        iterations,
        salt,
    } = WrappingKey::generate_pbkdf2(Digest::Sha1, 5, 3).unwrap();

    assert_eq!(digest, Digest::Sha1);
    assert_eq!(iterations, 5);
    assert_eq!(salt.len(), 3); // salt filled with random data
    assert_eq!(salt, &RND[..3]);
}

#[test]
fn pbkdf2_create_wrapping_key_empty_password() {
    let wkey_data = WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![1, 2, 3],
    };

    if let Error::InvalArg(err) = wkey_data.create_wrapping_key(b"").unwrap_err() {
        assert_eq!(err, "invalid password, cannot be empty");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn pbkdf2_create_wrapping_key_empty_salt() {
    let wkey_data = WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![],
    };

    if let Error::InvalArg(err) = wkey_data.create_wrapping_key(b"123").unwrap_err() {
        assert_eq!(err, "invalid salt, cannot be empty");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn pbkdf2_create_wrapping_key() {
    let wkey_data = WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![1, 2, 3],
    };

    let wkey = wkey_data.create_wrapping_key(b"123").unwrap();
    assert_eq!(
        wkey,
        vec![
            96, 23, 159, 91, 244, 187, 88, 88, 95, 129, 91, 252, 136, 14, 242, 207, 92, 3, 153, 56
        ]
    );
}

#[test]
fn from_binary_pbkdf2() {
    let mut c = Cursor::new(&[
        1, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
    ]);
    let wkey = Option::<WrappingKey>::from_binary(&mut c).unwrap();

    assert_eq!(
        wkey,
        Some(WrappingKey::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3]
        })
    );
}

#[test]
fn from_binary_none() {
    let mut c = Cursor::new(&[0xFF]);
    assert_eq!(Option::<WrappingKey>::from_binary(&mut c).unwrap(), None);
}

#[test]
fn from_binary_inval() {
    let mut c = Cursor::new(&[2]);
    let err = Option::<WrappingKey>::from_binary(&mut c).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(format!("{}", err), "invalid wrapping-key detected");
}

#[test]
fn into_binary_pbkdf2() {
    let mut c = Cursor::new(Vec::new());
    Some(WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 65536,
        salt: vec![1, 2, 3],
    })
    .into_binary(&mut c)
    .unwrap();

    assert_eq!(
        c.into_inner(),
        [1, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3]
    );
}

#[test]
fn into_binary_none() {
    let mut c = Cursor::new(Vec::new());
    let none: Option<WrappingKey> = None;

    none.into_binary(&mut c).unwrap();
    assert_eq!(c.into_inner(), [0xFF]);
}
