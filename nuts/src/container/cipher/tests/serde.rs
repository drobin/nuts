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

use nuts_bytes::bytes::{Error, Options};

use crate::container::cipher::Cipher;

#[test]
fn de_none() {
    let cipher = Options::new().from_bytes::<Cipher>(&[0]).unwrap();
    assert_eq!(cipher, Cipher::None);
}

#[test]
fn de_aes128_ctr() {
    let cipher = Options::new().from_bytes::<Cipher>(&[1]).unwrap();
    assert_eq!(cipher, Cipher::Aes128Ctr);
}

#[test]
fn de_invalid() {
    let err = Options::new().from_bytes::<Cipher>(&[2]).unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(
        msg,
        "invalid value: integer `2`, expected variant index 0 <= i < 2"
    );
}

#[test]
fn ser_none() {
    let vec = Options::new().to_vec(&Cipher::None).unwrap();
    assert_eq!(vec, [0]);
}

#[test]
fn ser_aes128_ctr() {
    let vec = Options::new().to_vec(&Cipher::Aes128Ctr).unwrap();
    assert_eq!(vec, [1]);
}
