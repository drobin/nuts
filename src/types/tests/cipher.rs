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
use crate::types::Cipher;

#[test]
fn from_string_none() {
    assert_eq!(Cipher::from_string("none").unwrap(), Cipher::None);
}

#[test]
fn from_aes128_ctr() {
    assert_eq!(
        Cipher::from_string("aes128-ctr").unwrap(),
        Cipher::Aes128Ctr
    );
}

#[test]
fn from_string_inval() {
    let err = Cipher::from_string("xxx").unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "invalid cipher: xxx");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn key_size_none() {
    assert_eq!(Cipher::None.key_size(), 0);
}

#[test]
fn key_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.key_size(), 16);
}

#[test]
fn iv_size_none() {
    assert_eq!(Cipher::None.iv_size(), 0);
}

#[test]
fn iv_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.iv_size(), 16);
}

#[test]
fn block_size_none() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn block_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn display_none() {
    assert_eq!(format!("{}", Cipher::None), "none");
}

#[test]
fn display_aes128_ctr() {
    assert_eq!(format!("{}", Cipher::Aes128Ctr), "aes128-ctr");
}
