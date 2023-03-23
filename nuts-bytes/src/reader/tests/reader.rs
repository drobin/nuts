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

use crate::error::{Error, IntType};
use crate::options::Int;
use crate::reader::Reader;

#[test]
fn remaining_bytes() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3]);

    for (offs, buf) in [vec![1, 2, 3], vec![2, 3], vec![3], vec![], vec![]]
        .iter()
        .enumerate()
    {
        reader.offs = offs;
        assert_eq!(reader.position(), offs);
        assert_eq!(reader.remaining_bytes(), buf);
    }
}

#[test]
fn fix_u8() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3]);

    assert_eq!(reader.read_u8().unwrap(), 1);
    assert_eq!(reader.position(), 1);
    assert_eq!(reader.remaining_bytes(), [2, 3]);

    assert_eq!(reader.read_u8().unwrap(), 2);
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining_bytes(), [3]);

    assert_eq!(reader.read_u8().unwrap(), 3);
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining_bytes(), []);

    let err = reader.read_u8().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining_bytes(), []);
}

#[test]
fn fix_u16() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3, 4, 5]);

    assert_eq!(reader.read_u16().unwrap(), 0x0102);
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining_bytes(), [3, 4, 5]);

    assert_eq!(reader.read_u16().unwrap(), 0x0304);
    assert_eq!(reader.position(), 4);
    assert_eq!(reader.remaining_bytes(), [5]);

    let err = reader.read_u16().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 4);
    assert_eq!(reader.remaining_bytes(), [5]);
}

#[test]
fn fix_u32() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    assert_eq!(reader.read_u32().unwrap(), 0x01020304);
    assert_eq!(reader.position(), 4);
    assert_eq!(reader.remaining_bytes(), [5, 6, 7, 8, 9, 10, 11]);

    assert_eq!(reader.read_u32().unwrap(), 0x05060708);
    assert_eq!(reader.position(), 8);
    assert_eq!(reader.remaining_bytes(), [9, 10, 11]);

    let err = reader.read_u32().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 8);
    assert_eq!(reader.remaining_bytes(), [9, 10, 11]);
}

#[test]
fn fix_u64() {
    let mut reader = Reader::new(
        Int::Fix,
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ],
    );

    assert_eq!(reader.read_u64().unwrap(), 0x0102030405060708);
    assert_eq!(reader.position(), 8);
    assert_eq!(
        reader.remaining_bytes(),
        [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,]
    );

    assert_eq!(reader.read_u64().unwrap(), 0x090A0B0C0D0E0F10);
    assert_eq!(reader.position(), 16);
    assert_eq!(reader.remaining_bytes(), [17, 18, 19, 20, 21, 22, 23]);

    let err = reader.read_u64().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 16);
    assert_eq!(reader.remaining_bytes(), [17, 18, 19, 20, 21, 22, 23]);
}

#[test]
fn fix_u128() {
    let mut reader = Reader::new(
        Int::Fix,
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47,
        ],
    );

    assert_eq!(
        reader.read_u128().unwrap(),
        0x0102030405060708090A0B0C0D0E0F10
    );
    assert_eq!(reader.position(), 16);
    assert_eq!(
        reader.remaining_bytes(),
        [
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
            39, 40, 41, 42, 43, 44, 45, 46, 47,
        ]
    );

    assert_eq!(
        reader.read_u128().unwrap(),
        0x1112131415161718191a1b1c1d1e1f20
    );
    assert_eq!(reader.position(), 32);
    assert_eq!(
        reader.remaining_bytes(),
        [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,]
    );

    let err = reader.read_u128().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 32);
    assert_eq!(
        reader.remaining_bytes(),
        [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,]
    );
}

#[test]
fn var_u8() {
    let mut reader = Reader::new(Int::Var, &[1, 2, 3]);

    assert_eq!(reader.read_u8().unwrap(), 1);
    assert_eq!(reader.position(), 1);
    assert_eq!(reader.remaining_bytes(), [2, 3]);

    assert_eq!(reader.read_u8().unwrap(), 2);
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining_bytes(), [3]);

    assert_eq!(reader.read_u8().unwrap(), 3);
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining_bytes(), []);

    let err = reader.read_u8().unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining_bytes(), []);
}

#[test]
fn var_u16() {
    for (buf, n) in [
        (vec![0], 0),
        (vec![64], 64),
        (vec![250], 250),
        (vec![251, 0, 0], 0),
        (vec![251, 0, 0xff], 0xff),
        (vec![251, 0xff, 0xff], 0xffff),
    ] {
        let mut reader = Reader::new(Int::Var, &buf);
        assert_eq!(reader.read_u16().unwrap(), n);
        assert_eq!(reader.position(), buf.len());
    }

    for (buf, t) in [
        (vec![252], IntType::U32),
        (vec![253], IntType::U64),
        (vec![254], IntType::U128),
    ] {
        let mut reader = Reader::new(Int::Var, &buf);
        let err = reader.read_u16().unwrap_err();
        assert_eq!(reader.position(), 1);
        assert_eq!(
            err,
            Error::InvalidInteger {
                expected: IntType::U16,
                found: t
            }
        )
    }
}

#[test]
fn var_u32() {
    for (buf, n) in [
        (vec![0], 0),
        (vec![64], 64),
        (vec![250], 250),
        (vec![251, 0, 0], 0),
        (vec![251, 0, 0xff], 0xff),
        (vec![251, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0, 0, 0], 0),
        (vec![252, 0, 0, 0, 0xff], 0xff),
        (vec![252, 0, 0, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0xff, 0xff, 0xff], 0xffffff),
        (vec![252, 0xff, 0xff, 0xff, 0xff], 0xffffffff),
    ] {
        let mut reader = Reader::new(Int::Var, &buf);
        assert_eq!(reader.read_u32().unwrap(), n);
        assert_eq!(reader.position(), buf.len());
    }

    for (buf, t) in [(vec![253], IntType::U64), (vec![254], IntType::U128)] {
        let mut reader = Reader::new(Int::Var, &buf);
        let err = reader.read_u32().unwrap_err();
        assert_eq!(reader.position(), 1);
        assert_eq!(
            err,
            Error::InvalidInteger {
                expected: IntType::U32,
                found: t
            }
        )
    }
}

#[test]
fn var_u64() {
    for (buf, n) in [
        (vec![0], 0),
        (vec![64], 64),
        (vec![250], 250),
        (vec![251, 0, 0], 0),
        (vec![251, 0, 0xff], 0xff),
        (vec![251, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0, 0, 0], 0),
        (vec![252, 0, 0, 0, 0xff], 0xff),
        (vec![252, 0, 0, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0xff, 0xff, 0xff], 0xffffff),
        (vec![252, 0xff, 0xff, 0xff, 0xff], 0xffffffff),
        (vec![253, 0, 0, 0, 0, 0, 0, 0, 0], 0),
        (vec![253, 0, 0, 0, 0, 0, 0, 0, 0xff], 0xff),
        (vec![253, 0, 0, 0, 0, 0, 0, 0xff, 0xff], 0xffff),
        (vec![253, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff], 0xffffff),
        (vec![253, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff], 0xffffffff),
        (
            vec![253, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff],
            0xffffffffff,
        ),
        (
            vec![253, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            0xffffffffffff,
        ),
        (
            vec![253, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            0xffffffffffffff,
        ),
        (
            vec![253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            0xffffffffffffffff,
        ),
    ] {
        let mut reader = Reader::new(Int::Var, &buf);
        assert_eq!(reader.read_u64().unwrap(), n);
        assert_eq!(reader.position(), buf.len());
    }

    for (buf, t) in [(vec![254], IntType::U128)] {
        let mut reader = Reader::new(Int::Var, &buf);
        let err = reader.read_u64().unwrap_err();
        assert_eq!(reader.position(), 1);
        assert_eq!(
            err,
            Error::InvalidInteger {
                expected: IntType::U32,
                found: t
            }
        )
    }
}

#[test]
fn var_u128() {
    for (buf, n) in [
        (vec![0], 0),
        (vec![64], 64),
        (vec![250], 250),
        (vec![251, 0, 0], 0),
        (vec![251, 0, 0xff], 0xff),
        (vec![251, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0, 0, 0], 0),
        (vec![252, 0, 0, 0, 0xff], 0xff),
        (vec![252, 0, 0, 0xff, 0xff], 0xffff),
        (vec![252, 0, 0xff, 0xff, 0xff], 0xffffff),
        (vec![252, 0xff, 0xff, 0xff, 0xff], 0xffffffff),
        (vec![253, 0, 0, 0, 0, 0, 0, 0, 0], 0),
        (vec![253, 0, 0, 0, 0, 0, 0, 0, 0xff], 0xff),
        (vec![253, 0, 0, 0, 0, 0, 0, 0xff, 0xff], 0xffff),
        (vec![253, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff], 0xffffff),
        (vec![253, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff], 0xffffffff),
        (vec![254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
        (
            vec![254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff],
            0xff,
        ),
        (
            vec![254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff],
            0xffff,
        ),
        (
            vec![254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff],
            0xffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            0xffffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff,
            ],
            0xffffffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff,
            ],
            0xffffffffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
            0xffffffffffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
            0xffffffffffffffffffffffffffff,
        ),
        (
            vec![
                254, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            0xffffffffffffffffffffffffffffff,
        ),
        (
            vec![
                254, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff,
            ],
            0xffffffffffffffffffffffffffffffff,
        ),
        (
            vec![253, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff],
            1099511627775,
        ),
        (
            vec![253, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            281474976710655,
        ),
        (
            vec![253, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            72057594037927935,
        ),
        (
            vec![253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            18446744073709551615,
        ),
    ] {
        let mut reader = Reader::new(Int::Var, &buf);
        assert_eq!(reader.read_u128().unwrap(), n);
        assert_eq!(reader.position(), buf.len());
    }
}

#[test]
fn bytes() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(0).unwrap(), []);
    assert_eq!(reader.remaining_bytes(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(1).unwrap(), [1]);
    assert_eq!(reader.remaining_bytes(), [2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(2).unwrap(), [2, 3]);
    assert_eq!(reader.remaining_bytes(), [4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(3).unwrap(), [4, 5, 6]);
    assert_eq!(reader.remaining_bytes(), [7, 8, 9]);

    let err = reader.read_bytes(4).unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(reader.remaining_bytes(), [7, 8, 9]);
}

#[test]
fn bytes_to() {
    let mut reader = Reader::new(Int::Fix, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(reader.position(), 0);
    assert_eq!(reader.remaining_bytes(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 1];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [1]);
    assert_eq!(reader.position(), 1);
    assert_eq!(reader.remaining_bytes(), [2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 2];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [2, 3]);
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining_bytes(), [4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 3];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [4, 5, 6]);
    assert_eq!(reader.position(), 6);
    assert_eq!(reader.remaining_bytes(), [7, 8, 9]);

    let mut buf = [0; 4];
    let err = reader.read_bytes_to(&mut buf).unwrap_err();
    assert_eq!(err, Error::Eof);
    assert_eq!(buf, [0, 0, 0, 0]);
    assert_eq!(reader.position(), 6);
    assert_eq!(reader.remaining_bytes(), [7, 8, 9]);
}
