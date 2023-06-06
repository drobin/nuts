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

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::Error;
use crate::reader::Reader;
use crate::source::BufferSource;
use crate::{assert_error, assert_error_eq};

fn setup(bytes: &[u8]) -> Reader<BufferSource> {
    Reader::new(BufferSource::new(bytes))
}

#[test]
fn bool() {
    for buf in [[1], [2]] {
        let mut reader = setup(&buf);
        assert_eq!(bool::deserialize(&mut reader).unwrap(), true);
    }

    let mut reader = setup(&[0]);
    assert_eq!(bool::deserialize(&mut reader).unwrap(), false);
}

#[test]
fn i8() {
    for (buf, n) in [([0xff], -1), ([0], 0), ([1], 1)] {
        let mut reader = setup(&buf);
        assert_eq!(i8::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn u8() {
    for (buf, n) in [([0], 0), ([1], 1), ([2], 2)] {
        let mut reader = setup(&buf);
        assert_eq!(u8::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn i16() {
    for (buf, n) in [([0xff, 0xff], -1), ([0x00, 0x00], 0), ([0x00, 0x01], 1)] {
        let mut reader = setup(&buf);
        assert_eq!(i16::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn u16() {
    for (buf, n) in [([0x00, 0x00], 0), ([0x00, 0x01], 1), ([0x00, 0x02], 2)] {
        let mut reader = setup(&buf);
        assert_eq!(u16::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn i32() {
    for (buf, n) in [
        ([0xff, 0xff, 0xff, 0xff], -1),
        ([0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x01], 1),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(i32::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn u32() {
    for (buf, n) in [
        ([0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x01], 1),
        ([0x00, 0x00, 0x00, 0x02], 2),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(u32::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn i64() {
    for (buf, n) in [
        ([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], -1),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 1),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(i64::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn u64() {
    for (buf, n) in [
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 1),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02], 2),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(u64::deserialize(&mut reader).unwrap(), n);
    }
}

#[test]
fn i128() {
    let mut reader = setup(&[0]);
    let err = i128::deserialize(&mut reader).unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "i128 is not supported"));
}

#[test]
fn u128() {
    let mut reader = setup(&[0]);
    let err = u128::deserialize(&mut reader).unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "u128 is not supported"));
}

#[test]
fn char() {
    let mut reader = setup(&[0x00, 0x01, 0xF4, 0xAF]);
    assert_eq!(char::deserialize(&mut reader).unwrap(), '💯');

    let mut reader = setup(&[0x00, 0x11, 0x00, 0x00]);
    let err = char::deserialize(&mut reader).unwrap_err();
    assert_error_eq!(err, Error::InvalidChar(|n| 0x110000));
}

#[test]
fn str() {
    for (bytes, str) in [
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], ""),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'a'],
            "a",
        ),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, b'a', b'b'],
            "ab",
        ),
        (
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'a', b'b', b'c',
            ],
            "abc",
        ),
    ] {
        let mut reader = setup(&bytes);
        assert_eq!(<&str>::deserialize(&mut reader).unwrap(), str);
    }

    let mut reader = setup(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0, 159, 146, 150,
    ]);
    let err = <&str>::deserialize(&mut reader).unwrap_err();
    assert_error!(
        err,
        Error::InvalidString(
            |cause| cause.to_string() == "invalid utf-8 sequence of 1 bytes from index 1"
        )
    );
}

#[test]
fn string() {
    for (bytes, str) in [
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], ""),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, b'a'],
            "a",
        ),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, b'a', b'b'],
            "ab",
        ),
        (
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, b'a', b'b', b'c',
            ],
            "abc",
        ),
    ] {
        let mut reader = setup(&bytes);
        assert_eq!(String::deserialize(&mut reader).unwrap(), str);
    }

    let mut reader = setup(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0, 159, 146, 150,
    ]);
    let err = String::deserialize(&mut reader).unwrap_err();
    assert_error!(
        err,
        Error::InvalidString(
            |cause| cause.to_string() == "invalid utf-8 sequence of 1 bytes from index 1"
        )
    );
}

#[test]
fn array() {
    let mut reader = setup(&[]);
    assert_eq!(<[u16; 0]>::deserialize(&mut reader).unwrap(), []);

    let mut reader = setup(&[0x00, 0x01]);
    assert_eq!(<[u16; 1]>::deserialize(&mut reader).unwrap(), [1]);

    let mut reader = setup(&[0x00, 0x01, 0x00, 0x02]);
    assert_eq!(<[u16; 2]>::deserialize(&mut reader).unwrap(), [1, 2]);

    let mut reader = setup(&[0x00, 0x01, 0x00, 0x02, 0x00, 0x03]);
    assert_eq!(<[u16; 3]>::deserialize(&mut reader).unwrap(), [1, 2, 3]);
}

#[test]
fn bytes() {
    for (buf, bytes) in [
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], vec![]),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 1],
            vec![1],
        ),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 1, 2],
            vec![1, 2],
        ),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3],
            vec![1, 2, 3],
        ),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(<&[u8]>::deserialize(&mut reader).unwrap(), bytes);
    }
}

#[test]
fn vec() {
    for (buf, bytes) in [
        (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], vec![]),
        (
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01],
            vec![1],
        ),
        (
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02,
            ],
            vec![1, 2],
        ),
        (
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03,
            ],
            vec![1, 2, 3],
        ),
    ] {
        let mut reader = setup(&buf);
        assert_eq!(<Vec<u16>>::deserialize(&mut reader).unwrap(), bytes);
    }
}

#[test]
fn option() {
    for buf in [[0x01, 0x00, 0x01], [0x02, 0x00, 0x01]] {
        let mut reader = setup(&buf);
        assert_eq!(<Option<u16>>::deserialize(&mut reader).unwrap(), Some(1));
    }

    let mut reader = setup(&[0]);
    assert_eq!(<Option<u16>>::deserialize(&mut reader).unwrap(), None);
}

fn sorted_keys<K: Ord, V>(m: &HashMap<K, V>) -> Vec<&K> {
    let mut keys = m.keys().collect::<Vec<&K>>();

    keys.sort();

    keys
}

#[test]
fn map() {
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let m = HashMap::<u8, u16>::deserialize(&mut reader).unwrap();
    assert!(m.is_empty());

    let mut reader = setup(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x12, 0x67,
    ]);
    let m = HashMap::<u8, u16>::deserialize(&mut reader).unwrap();
    assert_eq!(sorted_keys(&m), [&1]);
    assert_eq!(m.get(&1).unwrap(), &4711);

    let mut reader = setup(&[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x12, 0x67, 0x02, 0x02, 0x9A,
    ]);
    let m = HashMap::<u8, u16>::deserialize(&mut reader).unwrap();
    assert_eq!(sorted_keys(&m), [&1, &2]);
    assert_eq!(m.get(&1).unwrap(), &4711);
    assert_eq!(m.get(&2).unwrap(), &666);
}

#[test]
fn unit() {
    let mut reader = setup(&[]);
    assert_eq!(<()>::deserialize(&mut reader).unwrap(), ());
}

#[test]
fn r#struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct UnitStruct;

    #[derive(Debug, Deserialize, PartialEq)]
    struct NewTypeStruct(u16);

    #[derive(Debug, Deserialize, PartialEq)]
    struct TupleStruct(u16, u32);

    #[derive(Debug, Deserialize, PartialEq)]
    struct Struct {
        f1: u16,
        f2: u32,
    }

    // unit-struct
    let mut reader = setup(&[]);
    assert_eq!(UnitStruct::deserialize(&mut reader).unwrap(), UnitStruct);

    // newtype-struct
    let mut reader = setup(&[0x12, 0x67]);
    assert_eq!(
        NewTypeStruct::deserialize(&mut reader).unwrap(),
        NewTypeStruct(4711)
    );

    // tuple-struct
    let mut reader = setup(&[0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);
    assert_eq!(
        TupleStruct::deserialize(&mut reader).unwrap(),
        TupleStruct(4711, 666)
    );

    // struct
    let mut reader = setup(&[0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);
    assert_eq!(
        Struct::deserialize(&mut reader).unwrap(),
        Struct { f1: 4711, f2: 666 }
    );
}

#[test]
fn r#enum() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum Enum {
        V1,
        V2(u16),
        V3(u16, u32),
        V4 { f1: u16, f2: u32 },
    }

    // unit-variant
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x00]);
    assert_eq!(Enum::deserialize(&mut reader).unwrap(), Enum::V1);

    // newtype-variant
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x01, 0x12, 0x67]);
    assert_eq!(Enum::deserialize(&mut reader).unwrap(), Enum::V2(4711));

    // tuple-variant
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x02, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);
    assert_eq!(Enum::deserialize(&mut reader).unwrap(), Enum::V3(4711, 666));

    // struct-variant
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x03, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);
    assert_eq!(
        Enum::deserialize(&mut reader).unwrap(),
        Enum::V4 { f1: 4711, f2: 666 }
    );

    // invalid index
    let mut reader = setup(&[0x00, 0x00, 0x00, 0x04]);
    let err = Enum::deserialize(&mut reader).unwrap_err();
    assert_error_eq!(
        err,
        Error::Serde(|msg| "invalid value: integer `4`, expected variant index 0 <= i < 4")
    );
}
