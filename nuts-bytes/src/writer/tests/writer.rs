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
use crate::options::Int;
use crate::target::{BufferTarget, VecTarget};
use crate::writer::Writer;

fn setup_vec_fix() -> Writer<VecTarget> {
    Writer::new(Int::Fix, VecTarget::new(vec![]))
}

fn setup_slice_fix(target: &mut [u8]) -> Writer<BufferTarget> {
    Writer::new(Int::Fix, BufferTarget::new(target))
}

fn setup_vec_var() -> Writer<VecTarget> {
    Writer::new(Int::Var, VecTarget::new(vec![]))
}

fn setup_slice_var(target: &mut [u8]) -> Writer<BufferTarget> {
    Writer::new(Int::Var, BufferTarget::new(target))
}

#[test]
fn bytes_vec() {
    let mut writer = setup_vec_fix();

    writer.write_bytes(&[]).unwrap();
    assert_eq!(writer.as_ref().as_ref(), []);

    writer.write_bytes(&[1]).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1]);

    writer.write_bytes(&[2, 3]).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);

    writer.write_bytes(&[4, 5, 6]).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn bytes_slice() {
    let mut buf = [0; 9];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_bytes(&[]).unwrap();
    assert_eq!(writer.as_ref().position(), 0);
    assert_eq!(writer.as_ref().as_ref(), [0, 0, 0, 0, 0, 0, 0, 0, 0]);

    writer.write_bytes(&[1]).unwrap();
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 0, 0, 0, 0, 0, 0, 0, 0]);

    writer.write_bytes(&[2, 3]).unwrap();
    assert_eq!(writer.as_ref().position(), 3);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 0, 0, 0, 0, 0, 0]);

    writer.write_bytes(&[4, 5, 6]).unwrap();
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6, 0, 0, 0]);

    let err = writer.write_bytes(&[7, 8, 9, 10]).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 6);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3, 4, 5, 6, 0, 0, 0]);
}

#[test]
fn fix_u8_vec() {
    let mut writer = setup_vec_fix();

    writer.write_u8(1).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1,]);
    writer.write_u8(2).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2,]);
    writer.write_u8(3).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);
}

#[test]
fn fix_u8_slice() {
    let mut buf = [0; 2];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_u8(1).unwrap();
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 0]);

    writer.write_u8(2).unwrap();
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);

    let err = writer.write_u8(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);
}

#[test]
fn fix_u16_vec() {
    let mut writer = setup_vec_fix();

    writer.write_u16(1).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01]);
    writer.write_u16(2).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02]);
}

#[test]
fn fix_u16_slice() {
    let mut buf = [b'x'; 5];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_u16(1).unwrap();
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, b'x', b'x', b'x']);

    writer.write_u16(2).unwrap();
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02, b'x']);

    let err = writer.write_u16(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02, b'x']);
}

#[test]
fn fix_u32_vec() {
    let mut writer = setup_vec_fix();

    writer.write_u32(1).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x00, 0x00, 0x01,]);
    writer.write_u32(2).unwrap();
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]
    );
}

#[test]
fn fix_u32_slice() {
    let mut buf = [b'x'; 11];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_u32(1).unwrap();
    assert_eq!(writer.as_ref().position(), 4);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x', b'x']
    );

    writer.write_u32(2).unwrap();
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
fn fix_u64_vec() {
    let mut writer = setup_vec_fix();

    writer.write_u64(1).unwrap();
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,]
    );
    writer.write_u64(2).unwrap();
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02
        ]
    );
}

#[test]
fn fix_u64_slice() {
    let mut buf = [b'x'; 23];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_u64(1).unwrap();
    assert_eq!(writer.as_ref().position(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'
        ]
    );

    writer.write_u64(2).unwrap();
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
fn fix_u128_vec() {
    let mut writer = setup_vec_fix();

    writer.write_u128(1).unwrap();
    assert_eq!(
        writer.as_ref().as_ref(),
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01,
        ]
    );
    writer.write_u128(2).unwrap();
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
fn fix_u128_slice() {
    let mut buf = [b'x'; 47];
    let mut writer = setup_slice_fix(&mut buf);

    writer.write_u128(1).unwrap();
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

    writer.write_u128(2).unwrap();
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

#[test]
fn var_u8_vec() {
    let mut writer = setup_vec_var();

    writer.write_u8(1).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1]);
    writer.write_u8(2).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);
    writer.write_u8(3).unwrap();
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);
}

#[test]
fn var_u8_slice() {
    let mut buf = [0; 2];
    let mut writer = setup_slice_var(&mut buf);

    writer.write_u8(1).unwrap();
    assert_eq!(writer.as_ref().position(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1, 0]);

    writer.write_u8(2).unwrap();
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);

    let err = writer.write_u8(3).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(writer.as_ref().position(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);
}

#[test]
fn var_u16_vec() {
    for (n, buf) in [
        (0, vec![0]),
        (64, vec![64]),
        (250, vec![250]),
        (251, vec![251, 0, 0xfb]),
        (0xff, vec![251, 0, 0xff]),
        (0xffff, vec![251, 0xff, 0xff]),
    ] {
        let mut writer = setup_vec_var();

        writer.write_u16(n).unwrap();
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn var_u16_slice() {
    for (n, buf, pos) in [
        (0, [0, b'x', b'x'], 1),
        (64, [64, b'x', b'x'], 1),
        (250, [250, b'x', b'x'], 1),
        (251, [251, 0, 0xfb], 3),
        (0xff, [251, 0, 0xff], 3),
        (0xffff, [251, 0xff, 0xff], 3),
    ] {
        let mut out = [b'x'; 3];
        let mut writer = setup_slice_var(&mut out);

        writer.write_u16(n).unwrap();
        assert_eq!(writer.as_ref().position(), pos);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }

    let mut buf = [b'x'; 0];
    let err = setup_slice_var(&mut buf).write_u16(0).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));

    let mut buf = [b'x'; 2];
    let err = setup_slice_var(&mut buf).write_u16(251).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x']);
}

#[test]
fn var_u32_vec() {
    for (n, buf) in [
        (0, vec![0]),
        (64, vec![64]),
        (250, vec![250]),
        (251, vec![251, 0, 0xfb]),
        (0xff, vec![251, 0, 0xff]),
        (0xffff, vec![251, 0xff, 0xff]),
        (0x010000, vec![252, 0x00, 0x01, 0x00, 0x00]),
        (0xffffff, vec![252, 0x00, 0xff, 0xff, 0xff]),
        (0xffffffff, vec![252, 0xff, 0xff, 0xff, 0xff]),
    ] {
        let mut writer = setup_vec_var();

        writer.write_u32(n).unwrap();
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn var_u32_slice() {
    for (n, buf, pos) in [
        (0, [0, b'x', b'x', b'x', b'x'], 1),
        (64, [64, b'x', b'x', b'x', b'x'], 1),
        (250, [250, b'x', b'x', b'x', b'x'], 1),
        (251, [251, 0, 0xfb, b'x', b'x'], 3),
        (0xff, [251, 0, 0xff, b'x', b'x'], 3),
        (0xffff, [251, 0xff, 0xff, b'x', b'x'], 3),
        (0x010000, [252, 0x00, 0x01, 0x00, 0x00], 5),
        (0xffffff, [252, 0x00, 0xff, 0xff, 0xff], 5),
        (0xffffffff, [252, 0xff, 0xff, 0xff, 0xff], 5),
    ] {
        let mut out = [b'x'; 5];
        let mut writer = setup_slice_var(&mut out);

        writer.write_u32(n).unwrap();
        assert_eq!(writer.as_ref().position(), pos);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }

    let mut buf = [b'x'; 0];
    let err = setup_slice_var(&mut buf).write_u32(0).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));

    let mut buf = [b'x'; 2];
    let err = setup_slice_var(&mut buf).write_u32(251).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x']);

    let mut buf = [b'x'; 4];
    let err = setup_slice_var(&mut buf).write_u32(0x010000).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x', b'x', b'x']);
}

#[test]
fn var_u64_vec() {
    for (n, buf) in [
        (0, vec![0]),
        (64, vec![64]),
        (250, vec![250]),
        (251, vec![251, 0, 0xfb]),
        (0xff, vec![251, 0, 0xff]),
        (0xffff, vec![251, 0xff, 0xff]),
        (0x010000, vec![252, 0x00, 0x01, 0x00, 0x00]),
        (0xffffff, vec![252, 0x00, 0xff, 0xff, 0xff]),
        (0xffffffff, vec![252, 0xff, 0xff, 0xff, 0xff]),
        (
            0x100000000,
            vec![253, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
        ),
        (
            0xffffffffff,
            vec![253, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffff,
            vec![253, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffffff,
            vec![253, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffffffff,
            vec![253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
    ] {
        let mut writer = setup_vec_var();

        writer.write_u64(n).unwrap();
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn var_u64_slice() {
    for (n, buf, pos) in [
        (0, [0, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'], 1),
        (64, [64, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'], 1),
        (
            250,
            [250, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'],
            1,
        ),
        (251, [251, 0, 0xfb, b'x', b'x', b'x', b'x', b'x', b'x'], 3),
        (0xff, [251, 0, 0xff, b'x', b'x', b'x', b'x', b'x', b'x'], 3),
        (
            0xffff,
            [251, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x'],
            3,
        ),
        (
            0x010000,
            [252, 0x00, 0x01, 0x00, 0x00, b'x', b'x', b'x', b'x'],
            5,
        ),
        (
            0xffffff,
            [252, 0x00, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x'],
            5,
        ),
        (
            0xffffffff,
            [252, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x'],
            5,
        ),
        (
            0x100000000,
            [253, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
            9,
        ),
        (
            0xffffffffff,
            [253, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff],
            9,
        ),
        (
            0xffffffffffff,
            [253, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            9,
        ),
        (
            0xffffffffffffff,
            [253, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            9,
        ),
        (
            0xffffffffffffffff,
            [253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            9,
        ),
    ] {
        let mut out = [b'x'; 9];
        let mut writer = setup_slice_var(&mut out);

        writer.write_u64(n).unwrap();
        assert_eq!(writer.as_ref().position(), pos);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }

    let mut buf = [b'x'; 0];
    let err = setup_slice_var(&mut buf).write_u64(0).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));

    let mut buf = [b'x'; 2];
    let err = setup_slice_var(&mut buf).write_u64(251).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x']);

    let mut buf = [b'x'; 4];
    let err = setup_slice_var(&mut buf).write_u64(0x010000).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x', b'x', b'x']);

    let mut buf = [b'x'; 8];
    let err = setup_slice_var(&mut buf)
        .write_u64(0x100000000)
        .unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x']);
}

#[test]
fn var_u128_vec() {
    for (n, buf) in [
        (0, vec![0]),
        (64, vec![64]),
        (250, vec![250]),
        (251, vec![251, 0, 0xfb]),
        (0xff, vec![251, 0, 0xff]),
        (0xffff, vec![251, 0xff, 0xff]),
        (0x010000, vec![252, 0x00, 0x01, 0x00, 0x00]),
        (0xffffff, vec![252, 0x00, 0xff, 0xff, 0xff]),
        (0xffffffff, vec![252, 0xff, 0xff, 0xff, 0xff]),
        (
            0x100000000,
            vec![253, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
        ),
        (
            0xffffffffff,
            vec![253, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffff,
            vec![253, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffffff,
            vec![253, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0xffffffffffffffff,
            vec![253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            0x10000000000000000,
            vec![
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
        ),
        (
            0xffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffffffffff,
            vec![
                254, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffffffffffff,
            vec![
                254, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
        (
            0xffffffffffffffffffffffffffffffff,
            vec![
                254, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
        ),
    ] {
        let mut writer = setup_vec_var();

        writer.write_u128(n).unwrap();
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn var_u128_slice() {
    for (n, buf, pos) in [
        (
            0,
            [
                0, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            1,
        ),
        (
            64,
            [
                64, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            1,
        ),
        (
            250,
            [
                250, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            1,
        ),
        (
            251,
            [
                251, 0, 0xfb, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            3,
        ),
        (
            0xff,
            [
                251, 0, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            3,
        ),
        (
            0xffff,
            [
                251, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            3,
        ),
        (
            0x010000,
            [
                252, 0x00, 0x01, 0x00, 0x00, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            5,
        ),
        (
            0xffffff,
            [
                252, 0x00, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            5,
        ),
        (
            0xffffffff,
            [
                252, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            5,
        ),
        (
            0x100000000,
            [
                253, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            9,
        ),
        (
            0xffffffffff,
            [
                253, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            9,
        ),
        (
            0xffffffffffff,
            [
                253, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            9,
        ),
        (
            0xffffffffffffff,
            [
                253, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            9,
        ),
        (
            0xffffffffffffffff,
            [
                253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, b'x', b'x', b'x', b'x', b'x',
                b'x', b'x', b'x',
            ],
            9,
        ),
        (
            0x10000000000000000,
            [
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
            17,
        ),
        (
            0xffffffffffffffffff,
            [
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffff,
            [
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffff,
            [
                254, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffffff,
            [
                254, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffffffff,
            [
                254, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffffffffff,
            [
                254, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffffffffffff,
            [
                254, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
        (
            0xffffffffffffffffffffffffffffffff,
            [
                254, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            17,
        ),
    ] {
        let mut out = [b'x'; 17];
        let mut writer = setup_slice_var(&mut out);

        writer.write_u128(n).unwrap();
        assert_eq!(writer.as_ref().position(), pos);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }

    let mut buf = [b'x'; 0];
    let err = setup_slice_var(&mut buf).write_u128(0).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));

    let mut buf = [b'x'; 2];
    let err = setup_slice_var(&mut buf).write_u128(251).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x']);

    let mut buf = [b'x'; 4];
    let err = setup_slice_var(&mut buf).write_u128(0x010000).unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x', b'x', b'x']);

    let mut buf = [b'x'; 8];
    let err = setup_slice_var(&mut buf)
        .write_u128(0x100000000)
        .unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(buf, [b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x']);

    let mut buf = [b'x'; 16];
    let err = setup_slice_var(&mut buf)
        .write_u128(0x10000000000000000)
        .unwrap_err();
    assert_error!(err, Error::NoSpace(|cause| cause.is_none()));
    assert_eq!(
        buf,
        [
            b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x',
            b'x', b'x'
        ]
    );
}
