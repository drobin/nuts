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

#[test]
fn de_sha1() {
    let buf = [0x00, 0x00, 0x00, 0x00];

    assert_eq!(
        Digest::get_from_buffer(&mut &buf[..]).unwrap(),
        Digest::Sha1
    );
}

#[test]
fn de_eof() {
    let buf = [0x00, 0x00, 0x00];
    let err = Digest::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn de_invalid() {
    let buf = [0x00, 0x00, 0x00, 0x01];
    let err = Digest::get_from_buffer(&mut &buf[..]).unwrap_err();

    assert_eq!(err.to_string(), "no Digest at 1");
}

#[test]
fn ser_sha1() {
    let mut buf = vec![];

    Digest::Sha1.put_into_buffer(&mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn ser_write_zero() {
    let mut buf = [0; 3];
    let err = Digest::Sha1.put_into_buffer(&mut &mut buf[..]).unwrap_err();

    assert!(matches!(err, BufferError::WriteZero));
}
