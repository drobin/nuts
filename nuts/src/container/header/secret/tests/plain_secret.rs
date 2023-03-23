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

use nuts_bytes::Error;

use crate::container::cipher::Cipher;
use crate::container::header::secret::tests::{plain_secret, PLAIN_SECRET, SECRET};
use crate::container::header::secret::{bytes_options, PlainSecret};
use crate::container::kdf::Kdf;
use crate::container::password::PasswordStore;
use crate::container::Digest;
use crate::memory::MemoryBackend;

#[test]
fn ser() {
    let plain_secret = plain_secret();
    let vec = bytes_options().to_vec(&plain_secret).unwrap();
    assert_eq!(vec, PLAIN_SECRET);
}

#[test]
fn de() {
    let out = bytes_options()
        .from_bytes::<PlainSecret>(&PLAIN_SECRET)
        .unwrap();
    assert_eq!(out, plain_secret());
}

#[test]
fn de_inval() {
    let mut vec = PLAIN_SECRET.to_vec();
    vec[0] += 1;

    let err = bytes_options().from_bytes::<PlainSecret>(&vec).unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(msg, "secret-magic mismatch");
}

#[test]
fn encrypt_none() {
    let cb = || panic!("callback should never be called");
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let secret = plain_secret()
        .encrypt::<MemoryBackend>(&mut store, Cipher::None, &Kdf::None, &[])
        .unwrap();
    assert_eq!(secret, PLAIN_SECRET);
}

#[test]
fn encrypt_some() {
    let cb = || Ok(vec![1, 2, 3]);
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let kdf = Kdf::pbkdf2(Digest::Sha1, 1, &[0]);
    let secret = plain_secret()
        .encrypt::<MemoryBackend>(&mut store, Cipher::Aes128Ctr, &kdf, &[1; 16])
        .unwrap();
    assert_eq!(secret, SECRET);
}
