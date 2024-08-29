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

use crate::buffer::BufferError;
use crate::cipher::Cipher;

#[test]
fn de_none() {
    let buf = [0x00, 0x00, 0x00, 0x00];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::None
    );
}

#[test]
fn de_aes128_ctr() {
    let buf = [0x00, 0x00, 0x00, 0x01];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes128Ctr
    );
}

#[test]
fn de_aes192_ctr() {
    let buf = [0x00, 0x00, 0x00, 0x03];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes192Ctr
    );
}

#[test]
fn de_aes256_ctr() {
    let buf = [0x00, 0x00, 0x00, 0x04];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes256Ctr
    );
}

#[test]
fn de_aes128_gcm() {
    let buf = [0x00, 0x00, 0x00, 0x02];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes128Gcm
    );
}

#[test]
fn de_aes192_gcm() {
    let buf = [0x00, 0x00, 0x00, 0x05];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes192Gcm
    );
}

#[test]
fn de_aes256_gcm() {
    let buf = [0x00, 0x00, 0x00, 0x06];
    assert_eq!(
        Cipher::get_from_buffer(&mut &buf[..]).unwrap(),
        Cipher::Aes256Gcm
    );
}

#[test]
fn de_eof() {
    let buf = [0x00, 0x00, 0x00];
    let err = Cipher::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn de_invalid() {
    let buf = [0x00, 0x00, 0x00, 0x07];
    let err = Cipher::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert_eq!(err.to_string(), "no Cipher at 7");
}

#[test]
fn ser_none() {
    let mut buf = vec![];

    Cipher::None.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn ser_aes128_ctr() {
    let mut buf = vec![];

    Cipher::Aes128Ctr.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn ser_aes192_ctr() {
    let mut buf = vec![];

    Cipher::Aes192Ctr.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x03]);
}

#[test]
fn ser_aes256_ctr() {
    let mut buf = vec![];

    Cipher::Aes256Ctr.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x04]);
}

#[test]
fn ser_aes128_gcm() {
    let mut buf = vec![];

    Cipher::Aes128Gcm.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x02]);
}

#[test]
fn ser_aes192_gcm() {
    let mut buf = vec![];

    Cipher::Aes192Gcm.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x05]);
}

#[test]
fn ser_aes256_gcm() {
    let mut buf = vec![];

    Cipher::Aes256Gcm.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x06]);
}

#[test]
fn ser_write_zero() {
    let mut buf = [0; 3];
    let err = Cipher::None.put_into_buffer(&mut &mut buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
}
