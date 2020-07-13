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
use crate::secret::Secret;
use crate::types::{Cipher, Options};

#[test]
fn cipher_none_create_hmac() {
    let options = Options::default_with_cipher(Cipher::None);
    let secret = Secret::create(&options).unwrap();
    let mut header = Header::create(&options).unwrap();

    header.create_hmac(&secret).unwrap();
    assert!(header.hmac.is_empty());
}

#[test]
fn cipher_some_create_hmac() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr);
    let secret = Secret::create(&options).unwrap();
    let mut header = Header::create(&options).unwrap();

    header.create_hmac(&secret).unwrap();
    assert_eq!(header.hmac.len() as u32, options.md.unwrap().size());
}

#[test]
fn cipher_none_verify_hmac() {
    let options = Options::default_with_cipher(Cipher::None);
    let secret = Secret::create(&options).unwrap();
    let header = Header::create(&options).unwrap();

    header.verify_hmac(&secret).unwrap();
}

#[test]
fn cipher_some_verify_hmac() {
    let options = Options::default_with_cipher(Cipher::None);
    let secret = Secret::create(&options).unwrap();
    let mut header = Header::create(&options).unwrap();

    header.create_hmac(&secret).unwrap();
    header.verify_hmac(&secret).unwrap();
}

#[test]
fn cipher_some_verify_hmac_failed() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr);
    let secret = Secret::create(&options).unwrap();
    let mut header = Header::create(&options).unwrap();

    header.create_hmac(&secret).unwrap();
    *header.hmac.get_mut(0).unwrap() += 1;

    let err = header.verify_hmac(&secret).unwrap_err();
    assert_eq!(format!("{:?}", err), "HmacMismatch");
}
