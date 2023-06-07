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

use serde::Serialize;
use std::collections::HashMap;

use crate::error::Error;
use crate::{assert_error_eq, Writer};

#[test]
fn bool() {
    for (t, buf) in [(true, [0x01]), (false, [0x00])] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&t).unwrap(), 1);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn i8() {
    for (n, buf) in [(-1i8, [0xff]), (0, [0]), (1, [1])] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), 1);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u8() {
    for n in 0u8..2 {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), 1);
        assert_eq!(writer.as_ref().as_ref(), [n]);
    }
}

#[test]
fn i16() {
    for (n, buf) in [(-1i16, [0xff, 0xff]), (0, [0x00, 0x00]), (1, [0x00, 0x01])] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u16() {
    for (n, buf) in [(0u16, [0x00, 0x00]), (1, [0x00, 0x01]), (2, [0x00, 0x02])] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn i32() {
    for (n, buf) in [
        (-1i32, [0xff, 0xff, 0xff, 0xff]),
        (0, [0x00, 0x00, 0x00, 0x00]),
        (1, [0x00, 0x00, 0x00, 0x01]),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u32() {
    for (n, buf) in [
        (0u32, [0x00, 0x00, 0x00, 0x00]),
        (1, [0x00, 0x00, 0x00, 0x01]),
        (2, [0x00, 0x00, 0x00, 0x02]),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn i64() {
    for (n, buf) in [
        (-1i64, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        (0, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u64() {
    for (n, buf) in [
        (0u64, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (1, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]),
        (2, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&n).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn i128() {
    let mut writer = Writer::new(vec![]);
    let err = writer.serialize(&0i128).unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "i128 is not supported"));
}

#[test]
fn u128() {
    let mut writer = Writer::new(vec![]);
    let err = writer.serialize(&0u128).unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "u128 is not supported"));
}

#[test]
fn char() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&'💯').unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0xF4, 0xAF]);
}

#[test]
fn str() {
    for (str, buf) in [
        ("", vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (
            "a",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'a'],
        ),
        (
            "ab",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, b'a', b'b'],
        ),
        (
            "abc",
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'a', b'b', b'c',
            ],
        ),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&str).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn string() {
    for (str, buf) in [
        ("", vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (
            "a",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'a'],
        ),
        (
            "ab",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, b'a', b'b'],
        ),
        (
            "abc",
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'a', b'b', b'c',
            ],
        ),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&str.to_string()).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn array() {
    let arr: [u16; 0] = [];
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&arr).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);

    let arr: [u16; 1] = [1];
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&arr).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01]);

    let arr: [u16; 2] = [1, 2];
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&arr).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0x00, 0x02]);

    let arr: [u16; 3] = [1, 2, 3];
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&arr).unwrap(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x01, 0x00, 0x02, 0x00, 0x03]
    );
}

#[test]
fn bytes() {
    for (bytes, buf) in [
        (vec![], vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (
            vec![1u8],
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 1],
        ),
        (
            vec![1, 2],
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 1, 2],
        ),
        (
            vec![1, 2, 3],
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3],
        ),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&bytes.as_slice()).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn vec() {
    for (bytes, buf) in [
        (vec![], vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (
            vec![1u16],
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01],
        ),
        (
            vec![1, 2],
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02,
            ],
        ),
        (
            vec![1, 2, 3],
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03,
            ],
        ),
    ] {
        let mut writer = Writer::new(vec![]);
        assert_eq!(writer.serialize(&bytes).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn option() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Some(1u16)).unwrap(), 3);
    assert_eq!(writer.as_ref().as_ref(), [0x01, 0x00, 0x01]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Option::<u16>::None).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0x00]);
}

#[test]
fn map() {
    let map = HashMap::<u8, u16>::new();
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&map).unwrap(), 8);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );

    let map = HashMap::<u8, u16>::from([(1, 4711)]);
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&map).unwrap(), 11);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x12, 0x67]
    );

    let map = HashMap::<u8, u16>::from([(1, 4711), (2, 666)]);
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&map).unwrap(), 14);
    assert_eq!(
        writer.as_ref()[0..8],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
    );

    if writer.as_ref()[8] == 0x01 {
        assert_eq!(writer.as_ref()[8..], [0x01, 0x12, 0x67, 0x02, 0x02, 0x9A]);
    } else {
        assert_eq!(writer.as_ref()[8..], [0x02, 0x02, 0x9A, 0x01, 0x12, 0x67]);
    }
}

#[test]
fn unit() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&()).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);
}

#[test]
fn r#struct() {
    #[derive(Serialize)]
    struct UnitStruct;

    #[derive(Serialize)]
    struct NewTypeStruct(u16);

    #[derive(Serialize)]
    struct TupleStruct(u16, u32);

    #[derive(Serialize)]
    struct Struct {
        f1: u16,
        f2: u32,
    }

    // unit-struct
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&UnitStruct).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);

    // newtype-struct
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&NewTypeStruct(4711)).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x12, 0x67]);

    // tuple-struct
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&TupleStruct(4711, 666)).unwrap(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]
    );

    // struct
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Struct { f1: 4711, f2: 666 }).unwrap(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]
    );
}

#[test]
fn r#enum() {
    #[derive(Serialize)]
    enum Enum {
        V1,
        V2(u16),
        V3(u16, u32),
        V4 { f1: u16, f2: u32 },
    }

    // unit-variant
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Enum::V1).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x00, 0x00, 0x00]);

    // newtype-variant
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Enum::V2(4711)).unwrap(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x01, 0x12, 0x67]
    );

    // tuple-variant
    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.serialize(&Enum::V3(4711, 666)).unwrap(), 10);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x02, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]
    );

    // struct-variant
    let mut writer = Writer::new(vec![]);
    assert_eq!(
        writer.serialize(&Enum::V4 { f1: 4711, f2: 666 }).unwrap(),
        10
    );
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x00, 0x00, 0x00, 0x03, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]
    );
}
