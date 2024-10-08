// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

use crate::cipher::Cipher;

#[test]
fn from_str_none() {
    assert_eq!("none".parse::<Cipher>().unwrap(), Cipher::None);
}

#[test]
fn from_str_aes128_ctr() {
    assert_eq!("aes128-ctr".parse::<Cipher>().unwrap(), Cipher::Aes128Ctr);
}

#[test]
fn from_str_aes192_ctr() {
    assert_eq!("aes192-ctr".parse::<Cipher>().unwrap(), Cipher::Aes192Ctr);
}

#[test]
fn from_str_aes256_ctr() {
    assert_eq!("aes256-ctr".parse::<Cipher>().unwrap(), Cipher::Aes256Ctr);
}

#[test]
fn from_str_aes128_gcm() {
    assert_eq!("aes128-gcm".parse::<Cipher>().unwrap(), Cipher::Aes128Gcm);
}

#[test]
fn from_str_aes192_gcm() {
    assert_eq!("aes192-gcm".parse::<Cipher>().unwrap(), Cipher::Aes192Gcm);
}

#[test]
fn from_str_aes256_gcm() {
    assert_eq!("aes256-gcm".parse::<Cipher>().unwrap(), Cipher::Aes256Gcm);
}

#[test]
fn from_str_invalid() {
    "xxx".parse::<Cipher>().unwrap_err();
}

#[test]
fn to_string_none() {
    assert_eq!(Cipher::None.to_string(), "none");
}

#[test]
fn to_string_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.to_string(), "aes128-ctr");
}

#[test]
fn to_string_aes192_ctr() {
    assert_eq!(Cipher::Aes192Ctr.to_string(), "aes192-ctr");
}

#[test]
fn to_string_aes256_ctr() {
    assert_eq!(Cipher::Aes256Ctr.to_string(), "aes256-ctr");
}

#[test]
fn to_string_aes128_gcm() {
    assert_eq!(Cipher::Aes128Gcm.to_string(), "aes128-gcm");
}

#[test]
fn to_string_aes192_gcm() {
    assert_eq!(Cipher::Aes192Gcm.to_string(), "aes192-gcm");
}

#[test]
fn to_string_aes256_gcm() {
    assert_eq!(Cipher::Aes256Gcm.to_string(), "aes256-gcm");
}
