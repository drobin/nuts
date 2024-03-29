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

use nuts_bytes::{Error, Reader, Writer};

use crate::cipher::Cipher;

#[test]
fn de_none() {
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x00].as_slice());
    assert_eq!(reader.read::<Cipher>().unwrap(), Cipher::None);
}

#[test]
fn de_aes128_ctr() {
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x01].as_slice());
    assert_eq!(reader.read::<Cipher>().unwrap(), Cipher::Aes128Ctr);
}

#[test]
fn de_aes128_gcm() {
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x02].as_slice());
    assert_eq!(reader.read::<Cipher>().unwrap(), Cipher::Aes128Gcm);
}

#[test]
fn de_invalid() {
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x03].as_slice());

    let err = reader.read::<Cipher>().unwrap_err();
    assert!(matches!(err, Error::InvalidVariantIndex(3)));
}

#[test]
fn ser_none() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Cipher::None).unwrap(), 4);
    assert_eq!(writer.into_target(), [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn ser_aes128_ctr() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Cipher::Aes128Ctr).unwrap(), 4);
    assert_eq!(writer.into_target(), [0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn ser_aes128_gcm() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Cipher::Aes128Gcm).unwrap(), 4);
    assert_eq!(writer.into_target(), [0x00, 0x00, 0x00, 0x02]);
}
