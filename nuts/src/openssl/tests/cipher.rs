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

use crate::error::Error;
use crate::openssl::cipher;
use crate::types::Cipher;

#[test]
fn cipher_none_encrypt() {
    let output = cipher(Cipher::None, true, b"123", b"", b"").unwrap();
    assert_eq!(output, vec![b'1', b'2', b'3']);
}

#[test]
fn cipher_none_encrypt_keys_ignored() {
    let output = cipher(Cipher::None, true, b"123", b"456", b"789").unwrap();
    assert_eq!(output, vec![b'1', b'2', b'3']);
}

#[test]
fn cipher_some_inval_key_size() {
    let key = [7; 15];
    let iv = [9; 16];

    if let Error::InvalArg(msg) = cipher(Cipher::Aes128Ctr, true, b"123", &key, &iv).unwrap_err() {
        assert_eq!(msg, "key too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn cipher_some_inval_iv_size() {
    let key = [7; 16];
    let iv = [9; 15];

    if let Error::InvalArg(msg) = cipher(Cipher::Aes128Ctr, true, b"123", &key, &iv).unwrap_err() {
        assert_eq!(msg, "iv too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn cipher_some_encrypt() {
    let key = [7; 16];
    let iv = [9; 16];
    let output = cipher(Cipher::Aes128Ctr, true, b"123", &key, &iv).unwrap();
    assert_eq!(output, vec![160, 125, 33]);
}

#[test]
fn cipher_some_encrypt_keys_too_long() {
    let key = [7; 17];
    let iv = [9; 17];
    let output = cipher(Cipher::Aes128Ctr, true, b"123", &key, &iv).unwrap();
    assert_eq!(output, vec![160, 125, 33]);
}

#[test]
fn cipher_none_decrypt() {
    let output = cipher(Cipher::None, false, b"123", b"", b"").unwrap();
    assert_eq!(output, vec![b'1', b'2', b'3']);
}

#[test]
fn cipher_none_decrypt_keys_ignored() {
    let output = cipher(Cipher::None, false, b"123", b"456", b"789").unwrap();
    assert_eq!(output, vec![b'1', b'2', b'3']);
}

#[test]
fn cipher_some_decrypt() {
    let key = [7; 16];
    let iv = [9; 16];
    let output = cipher(Cipher::Aes128Ctr, false, &vec![160, 125, 33], &key, &iv).unwrap();
    assert_eq!(output, b"123");
}

#[test]
fn cipher_some_decrypt_keys_too_long() {
    let key = [7; 17];
    let iv = [9; 17];
    let output = cipher(Cipher::Aes128Ctr, false, &vec![160, 125, 33], &key, &iv).unwrap();
    assert_eq!(output, b"123");
}
