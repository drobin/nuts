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

use std::rc::Rc;

use nuts_bytes::{Error as BytesError, Reader, Writer};

use crate::container::cipher::Cipher;
use crate::container::header::secret::tests::{plain_secret, PLAIN_SECRET, SECRET};
use crate::container::header::secret::Secret;
use crate::container::kdf::Kdf;
use crate::container::password::PasswordStore;
use crate::container::{Digest, HeaderError};
use crate::memory::MemoryBackend;

#[test]
fn ser_empty() {
    let secret = Secret(vec![]);
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&secret).unwrap(), 8);
    assert_eq!(
        writer.into_target(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
}

#[test]
fn ser() {
    let secret = Secret(vec![1, 2, 3]);
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&secret).unwrap(), 11);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 3]);
}

#[test]
fn de_empty() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 0].as_slice());
    let secret = reader.deserialize::<Secret>().unwrap();
    assert_eq!(secret, []);
}

#[test]
fn de() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 3].as_slice());
    let secret = reader.deserialize::<Secret>().unwrap();
    assert_eq!(secret, [1, 2, 3]);
}

#[test]
fn decrypt_none_valid() {
    let cb = || panic!("callback should never be called");
    let mut store = PasswordStore::new(Some(Rc::new(cb)));
    let secret = Secret(PLAIN_SECRET.to_vec());

    let out = secret
        .decrypt(&mut store, Cipher::None, &Kdf::None, &[])
        .unwrap();
    assert_eq!(out, plain_secret());
}

#[test]
fn decrypt_none_invalid() {
    let cb = || panic!("callback should never be called");
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let mut vec = PLAIN_SECRET.to_vec();
    vec[0] += 1; // make magics invalid

    let secret = Secret(vec);

    let err = secret
        .decrypt::<MemoryBackend>(&mut store, Cipher::None, &Kdf::None, &[])
        .unwrap_err();

    let err = into_error!(err, HeaderError::WrongPassword);
    let msg = into_error!(err, BytesError::Serde);
    assert_eq!(msg, "secret-magic mismatch");
}

#[test]
fn decrypt_some_valid() {
    let cb = || Ok(vec![1, 2, 3]);
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let secret = Secret(SECRET.to_vec());
    let kdf = Kdf::pbkdf2(Digest::Sha1, 1, &[0]);

    let out = secret
        .decrypt(&mut store, Cipher::Aes128Ctr, &kdf, &[1; 16])
        .unwrap();
    assert_eq!(out, plain_secret());
}

#[test]
fn decrypt_some_invalid() {
    let cb = || Ok(vec![b'x', 2, 3]);
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let secret = Secret(SECRET.to_vec());
    let kdf = Kdf::pbkdf2(Digest::Sha1, 1, &[0]);

    let err = secret
        .decrypt::<MemoryBackend>(&mut store, Cipher::Aes128Ctr, &kdf, &[1; 16])
        .unwrap_err();

    let err = into_error!(err, HeaderError::WrongPassword);
    let msg = into_error!(err, BytesError::Serde);
    assert_eq!(msg, "secret-magic mismatch");
}
