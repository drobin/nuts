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
use crate::digest::Digest;
use crate::kdf::Kdf;

#[test]
fn de_none() {
    let buf = [0x00, 0x00, 0x00, 0x00];

    assert_eq!(Kdf::get_from_buffer(&mut &buf[..]).unwrap(), Kdf::None);
}

#[test]
fn de_pbkdf2() {
    let buf = [
        0x00, 0x00, 0x00, 0x01, // pbkdf2 variant
        0x00, 0x00, 0x00, 0x00, // sha1
        0x00, 0x01, 0x00, 0x00, // iterations
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // salt
    ];

    assert_eq!(
        Kdf::get_from_buffer(&mut &buf[..]).unwrap(),
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3]
        }
    );
}

#[test]
fn de_eof() {
    let buf = [0x00, 0x00, 0x00];
    let err = Kdf::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn de_invalid() {
    let buf = [0x00, 0x00, 0x00, 0x02];
    let err = Kdf::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert_eq!(err.to_string(), "no Kdf at 2");
}

#[test]
fn ser_none() {
    let mut buf = vec![];

    Kdf::None.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00,]);
}

#[test]
fn ser_pbkdf2() {
    let mut buf = vec![];

    Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 65536,
        salt: vec![1, 2, 3],
    }
    .put_into_buffer(&mut buf)
    .unwrap();

    assert_eq!(
        buf,
        [
            0x00, 0x00, 0x00, 0x01, // pbkdf2 variant
            0x00, 0x00, 0x00, 0x00, // sha1
            0x000, 0x01, 0x00, 0x00, // iterations
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3 // salt
        ]
    );
}

#[test]
fn ser_write_zero() {
    let mut buf = [0; 3];
    let err = Kdf::None.put_into_buffer(&mut &mut buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
}
