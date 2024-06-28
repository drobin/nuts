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

use std::rc::Rc;

use crate::buffer::{FromBuffer, ToBuffer};
use crate::cipher::Cipher;
use crate::digest::Digest;
use crate::header::plain_secret::{Encryptor, PlainSecret};
use crate::header::HeaderError;
use crate::kdf::Kdf;
use crate::password::PasswordStore;

// key: AE 18 FF 41 77 79 0F 07 AB 11 E2 F1 8C 87 AD 9A
// iv: 01010101010101010101010101010101
const SECRET: [u8; 45] = [
    0x5c, 0x68, 0x30, 0x8f, 0x47, 0x19, 0xf4, 0x76, 0xf2, 0x72, 0xbc, 0x6, 0x1c, 0xf3, 0x58, 0xca,
    0x54, 0x2c, 0xca, 0xf8, 0xe6, 0x7d, 0xe1, 0xfb, 0xb4, 0xe1, 0x1c, 0xbe, 0xb7, 0x83, 0x54, 0x3b,
    0xec, 0x8c, 0xee, 0xac, 0x59, 0x21, 0x58, 0xb3, 0x71, 0x90, 0x41, 0x4c, 0xe4,
];

const PLAIN_SECRET: [u8; 45] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    0, 0, 0, 0, 0, 0, 0, 2, 1, 2, // key
    0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 5, // iv
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Some(top-id)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // settings (empty)
];

fn plain_secret() -> PlainSecret {
    PlainSecret {
        magics: [4711, 4711],
        key: vec![1, 2].into(),
        iv: vec![3, 4, 5].into(),
        userdata: vec![].into(),
        settings: vec![].into(),
    }
}

#[test]
fn ser() {
    let mut buf = vec![];

    plain_secret().to_buffer(&mut buf).unwrap();
    assert_eq!(buf, PLAIN_SECRET);
}

#[test]
fn de() {
    let out = PlainSecret::from_buffer(&mut &PLAIN_SECRET[..]).unwrap();

    assert_eq!(out, plain_secret());
}

#[test]
fn de_inval() {
    let mut vec = PLAIN_SECRET.to_vec();
    vec[0] += 1;

    let err = PlainSecret::from_buffer(&mut vec.as_slice()).unwrap_err();
    assert!(matches!(err, HeaderError::WrongPassword));
}

#[test]
fn encrypt_none() {
    let cb = || panic!("callback should never be called");
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let secret = plain_secret()
        .encrypt(&mut store, Cipher::None, &Kdf::None, &[])
        .unwrap();
    assert_eq!(secret, PLAIN_SECRET);
}

#[test]
fn encrypt_some() {
    let cb = || Ok(vec![1, 2, 3]);
    let mut store = PasswordStore::new(Some(Rc::new(cb)));

    let kdf = Kdf::pbkdf2(Digest::Sha1, 1, &[0]);
    let secret = plain_secret()
        .encrypt(&mut store, Cipher::Aes128Ctr, &kdf, &[1; 16])
        .unwrap();
    assert_eq!(secret, SECRET);
}
