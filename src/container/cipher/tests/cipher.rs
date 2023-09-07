// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use crate::container::cipher::{Cipher, CipherError};
use crate::tests::into_error;

#[test]
fn block_size_none() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn block_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn key_len_none() {
    assert_eq!(Cipher::None.key_len(), 0);
}

#[test]
fn key_len_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.key_len(), 16);
}

#[test]
fn iv_len_none() {
    assert_eq!(Cipher::None.iv_len(), 0);
}

#[test]
fn iv_len_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.iv_len(), 16);
}

#[test]
fn from_str_none() {
    assert_eq!("none".parse::<Cipher>().unwrap(), Cipher::None);
}

#[test]
fn from_str_aes128_ctr() {
    assert_eq!("aes128-ctr".parse::<Cipher>().unwrap(), Cipher::Aes128Ctr);
}

#[test]
fn from_str_invalid() {
    let err = "xxx".parse::<Cipher>().unwrap_err();
    #[allow(unreachable_patterns)]
    let str = into_error!(err, CipherError::Invalid);
    assert_eq!(str, "xxx");
}

#[test]
fn to_string_none() {
    assert_eq!(Cipher::None.to_string(), "none");
}

#[test]
fn to_string_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.to_string(), "aes128-ctr");
}
