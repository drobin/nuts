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

use crate::assert_error;
use crate::error::Error;
use crate::target::{BufferTarget, VecTarget};
use crate::writer::Writer;

fn setup_vec() -> Writer<VecTarget> {
    Writer::new(VecTarget::new(vec![]))
}

fn setup_slice(target: &mut [u8]) -> Writer<BufferTarget> {
    Writer::new(BufferTarget::new(target))
}

#[test]
fn bytes_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_bytes(&[]).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);

    assert_eq!(writer.write_bytes(&[1]).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1]);

    assert_eq!(writer.write_bytes(&[2, 3]).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);

    assert_eq!(writer.write_bytes(&[4, 5, 6]).unwrap(), 3);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn bytes_slice() {
    let mut buf = [0; 9];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_bytes(&[]).unwrap(), 0);
    assert_eq!(writer.as_ref().position(), 0);
    assert_eq!(writer.as_ref().as_ref(), [0, 0, 0, 0, 0, 0, 0, 0, 0]);

    assert_eq!(writer.write_bytes(&[1]).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 0, 0, 0, 0, 0, 0, 0, 0]);

    assert_eq!(writer.write_bytes(&[2, 3]).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 3);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 0, 0, 0, 0, 0, 0]);

    assert_eq!(writer.write_bytes(&[4, 5, 6]).unwrap(), 3);
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6, 0, 0, 0]);

    let err = writer.write_bytes(&[7, 8, 9, 10]).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6, 0, 0, 0]);
}

#[test]
fn i8_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_i8(-1).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0xff,]);
    assert_eq!(writer.write_i8(0).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00,]);
    assert_eq!(writer.write_i8(1).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00, 0x01]);
}

#[test]
fn i8_slice() {
    let mut buf = [0; 3];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_i8(-1).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00, 0x00]);

    assert_eq!(writer.write_i8(0).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00, 0x00]);

    assert_eq!(writer.write_i8(1).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 3);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00, 0x01]);

    let err = writer.write_i8(2).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 3);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0x00, 0x01]);
}

#[test]
fn u8_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_u8(1).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1,]);
    assert_eq!(writer.write_u8(2).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 2,]);
    assert_eq!(writer.write_u8(3).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);
}

#[test]
fn u8_slice() {
    let mut buf = [0; 2];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_u8(1).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 0]);

    assert_eq!(writer.write_u8(2).unwrap(), 1);
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);

    let err = writer.write_u8(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);
}

#[test]
fn i16_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_i16(-1).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0xff]);
    assert_eq!(writer.write_i16(0).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0xff, 0x00, 0x00]);
    assert_eq!(writer.write_i16(1).unwrap(), 2);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0x00, 0x00, 0x00, 0x01]
    );
}

#[test]
fn i16_slice() {
    let mut buf = [b'x'; 7];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_i16(-1).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, b'x', b'x', b'x', b'x', b'x']
    );

    assert_eq!(writer.write_i16(0).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0x00, 0x00, b'x', b'x', b'x']
    );

    assert_eq!(writer.write_i16(1).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0x00, 0x00, 0x00, 0x01, b'x']
    );

    let err = writer.write_i16(2).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0x00, 0x00, 0x00, 0x01, b'x']
    );
}

#[test]
fn u16_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_u16(1).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01]);
    assert_eq!(writer.write_u16(2).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02]);
}

#[test]
fn u16_slice() {
    let mut buf = [b'x'; 5];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_u16(1).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, b'x', b'x', b'x']);

    assert_eq!(writer.write_u16(2).unwrap(), 2);
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02, b'x']);

    let err = writer.write_u16(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02, b'x']);
}

#[test]
fn i32_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_i32(-1).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0xff, 0xff, 0xff, 0xff,]);
    assert_eq!(writer.write_i32(0).unwrap(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]
    );
    assert_eq!(writer.write_i32(1).unwrap(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
    );
}

#[test]
fn i32_slice() {
    let mut buf = [b'x'; 15];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_i32(-1).unwrap(), 4);
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x'
        ]
    );

    assert_eq!(writer.write_i32(0).unwrap(), 4);
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, b'x', b'x', b'x', b'x', b'x', b'x',
            b'x'
        ]
    );

    assert_eq!(writer.write_i32(1).unwrap(), 4);
    assert_eq!(writer.as_ref().position(), 12);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x',
            b'x'
        ]
    );

    let err = writer.write_i32(2).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 12);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x',
            b'x'
        ]
    );
}

#[test]
fn u32_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_u32(1).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x00, 0x00, 0x01,]);
    assert_eq!(writer.write_u32(2).unwrap(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]
    );
}

#[test]
fn u32_slice() {
    let mut buf = [b'x'; 11];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_u32(1).unwrap(), 4);
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x', b'x']
    );

    assert_eq!(writer.write_u32(2).unwrap(), 4);
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, b'x', b'x', b'x']
    );

    let err = writer.write_u32(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, b'x', b'x', b'x']
    );
}

#[test]
fn i64_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_i64(-1).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,]
    );
    assert_eq!(writer.write_i64(0).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00
        ]
    );
    assert_eq!(writer.write_i64(1).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
        ]
    );
}

#[test]
fn i64_slice() {
    let mut buf = [b'x'; 31];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_i64(-1).unwrap(), 8);
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_i64(0).unwrap(), 8);
    assert_eq!(writer.as_ref().position(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_i64(1).unwrap(), 8);
    assert_eq!(writer.as_ref().position(), 24);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x',
            b'x', b'x', b'x'
        ]
    );

    let err = writer.write_i64(2).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 24);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x',
            b'x', b'x', b'x'
        ]
    );
}

#[test]
fn u64_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_u64(1).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,]
    );
    assert_eq!(writer.write_u64(2).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02
        ]
    );
}

#[test]
fn u64_slice() {
    let mut buf = [b'x'; 23];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_u64(1).unwrap(), 8);
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_u64(2).unwrap(), 8);
    assert_eq!(writer.as_ref().position(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02, b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    let err = writer.write_u64(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02, b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );
}

#[test]
fn i128_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_i128(-1).unwrap(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ]
    );
    assert_eq!(writer.write_i128(0).unwrap(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00
        ]
    );
    assert_eq!(writer.write_i128(1).unwrap(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01
        ]
    );
}

#[test]
fn i128_slice() {
    let mut buf = [b'x'; 63];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_i128(-1).unwrap(), 16);
    assert_eq!(writer.as_ref().position(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_i128(0).unwrap(), 16);
    assert_eq!(writer.as_ref().position(), 32);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_i128(1).unwrap(), 16);
    assert_eq!(writer.as_ref().position(), 48);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    let err = writer.write_i128(2).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 48);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );
}

#[test]
fn u128_vec() {
    let mut writer = setup_vec();

    assert_eq!(writer.write_u128(1).unwrap(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ]
    );
    assert_eq!(writer.write_u128(2).unwrap(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02
        ]
    );
}

#[test]
fn u128_slice() {
    let mut buf = [b'x'; 47];
    let mut writer = setup_slice(&mut buf);

    assert_eq!(writer.write_u128(1).unwrap(), 16);
    assert_eq!(writer.as_ref().position(), 16);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x'
        ]
    );

    assert_eq!(writer.write_u128(2).unwrap(), 16);
    assert_eq!(writer.as_ref().position(), 32);

    let err = writer.write_u128(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 32);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x'
        ]
    );
}
