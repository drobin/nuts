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

use crate::digest::Digest;

#[test]
fn size_sha1() {
    assert_eq!(Digest::Sha1.size(), 20);
}

#[test]
fn size_sha224() {
    assert_eq!(Digest::Sha224.size(), 28);
}

#[test]
fn size_sha256() {
    assert_eq!(Digest::Sha256.size(), 32);
}

#[test]
fn size_sha384() {
    assert_eq!(Digest::Sha384.size(), 48);
}

#[test]
fn size_sha512() {
    assert_eq!(Digest::Sha512.size(), 64);
}

#[test]
fn from_str_sha1() {
    assert_eq!("sha1".parse::<Digest>().unwrap(), Digest::Sha1);
}

#[test]
fn from_str_sha224() {
    assert_eq!("sha224".parse::<Digest>().unwrap(), Digest::Sha224);
}

#[test]
fn from_str_sha256() {
    assert_eq!("sha256".parse::<Digest>().unwrap(), Digest::Sha256);
}

#[test]
fn from_str_sha384() {
    assert_eq!("sha384".parse::<Digest>().unwrap(), Digest::Sha384);
}

#[test]
fn from_str_sha512() {
    assert_eq!("sha512".parse::<Digest>().unwrap(), Digest::Sha512);
}

#[test]
fn from_str_invalid() {
    "xxx".parse::<Digest>().unwrap_err();
}

#[test]
fn to_string_sha1() {
    assert_eq!(Digest::Sha1.to_string(), "sha1");
}

#[test]
fn to_string_sha224() {
    assert_eq!(Digest::Sha224.to_string(), "sha224");
}

#[test]
fn to_string_sha256() {
    assert_eq!(Digest::Sha256.to_string(), "sha256");
}

#[test]
fn to_string_sha384() {
    assert_eq!(Digest::Sha384.to_string(), "sha384");
}

#[test]
fn to_string_sha512() {
    assert_eq!(Digest::Sha512.to_string(), "sha512");
}
