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

use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Error;
use crate::reader::Reader;
use crate::{assert_error, assert_error_eq};

#[test]
fn bool() {
    for buf in [[1], [2]] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<bool>().unwrap(), true);
    }

    let mut reader = Reader::new([0].as_slice());
    assert_eq!(reader.deserialize::<bool>().unwrap(), false);
}

#[test]
fn i8() {
    for (buf, n) in [([0xff], -1), ([0], 0), ([1], 1)] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<i8>().unwrap(), n);
    }
}

#[test]
fn u8() {
    for (buf, n) in [([0], 0), ([1], 1), ([2], 2)] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<u8>().unwrap(), n);
    }
}

#[test]
fn i16() {
    for (buf, n) in [([0xff, 0xff], -1), ([0x00, 0x00], 0), ([0x00, 0x01], 1)] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<i16>().unwrap(), n);
    }
}

#[test]
fn u16() {
    for (buf, n) in [([0x00, 0x00], 0), ([0x00, 0x01], 1), ([0x00, 0x02], 2)] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<u16>().unwrap(), n);
    }
}

#[test]
fn i32() {
    for (buf, n) in [
        ([0xff, 0xff, 0xff, 0xff], -1),
        ([0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x01], 1),
    ] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<i32>().unwrap(), n);
    }
}

#[test]
fn u32() {
    for (buf, n) in [
        ([0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x01], 1),
        ([0x00, 0x00, 0x00, 0x02], 2),
    ] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<u32>().unwrap(), n);
    }
}

#[test]
fn i64() {
    for (buf, n) in [
        ([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], -1),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 1),
    ] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<i64>().unwrap(), n);
    }
}

#[test]
fn u64() {
    for (buf, n) in [
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 1),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02], 2),
    ] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<u64>().unwrap(), n);
    }
}

#[test]
fn i128() {
    let mut reader = Reader::new([0].as_slice());
    let err = reader.deserialize::<i128>().unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "i128 is not supported"));
}

#[test]
fn u128() {
    let mut reader = Reader::new([0].as_slice());
    let err = reader.deserialize::<u128>().unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "u128 is not supported"));
}

#[test]
fn char() {
    let mut reader = Reader::new([0x00, 0x01, 0xF4, 0xAF].as_slice());
    assert_eq!(reader.deserialize::<char>().unwrap(), '💯');

    let mut reader = Reader::new([0x00, 0x11, 0x00, 0x00].as_slice());
    let err = reader.deserialize::<char>().unwrap_err();
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
        let mut reader = Reader::new(bytes.as_slice());
        assert_eq!(reader.deserialize::<&str>().unwrap(), str);
    }

    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0, 159, 146, 150,
        ]
        .as_slice(),
    );
    let err = reader.deserialize::<&str>().unwrap_err();
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
        let mut reader = Reader::new(bytes.as_slice());
        assert_eq!(reader.deserialize::<String>().unwrap(), str);
    }

    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0, 159, 146, 150,
        ]
        .as_slice(),
    );
    let err = reader.deserialize::<String>().unwrap_err();
    assert_error!(
        err,
        Error::InvalidString(
            |cause| cause.to_string() == "invalid utf-8 sequence of 1 bytes from index 1"
        )
    );
}

#[test]
fn array() {
    let mut reader = Reader::new([].as_slice());
    assert_eq!(reader.deserialize::<[u16; 0]>().unwrap(), []);

    let mut reader = Reader::new([0x00, 0x01].as_slice());
    assert_eq!(reader.deserialize::<[u16; 1]>().unwrap(), [1]);

    let mut reader = Reader::new([0x00, 0x01, 0x00, 0x02].as_slice());
    assert_eq!(reader.deserialize::<[u16; 2]>().unwrap(), [1, 2]);

    let mut reader = Reader::new([0x00, 0x01, 0x00, 0x02, 0x00, 0x03].as_slice());
    assert_eq!(reader.deserialize::<[u16; 3]>().unwrap(), [1, 2, 3]);
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
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<&[u8]>().unwrap(), bytes);
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
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<Vec<u16>>().unwrap(), bytes);
    }
}

#[test]
fn option() {
    for buf in [[0x01, 0x00, 0x01], [0x02, 0x00, 0x01]] {
        let mut reader = Reader::new(buf.as_slice());
        assert_eq!(reader.deserialize::<Option<u16>>().unwrap(), Some(1));
    }

    let mut reader = Reader::new([0].as_slice());
    assert_eq!(reader.deserialize::<Option<u16>>().unwrap(), None);
}

fn sorted_keys<K: Ord, V>(m: &HashMap<K, V>) -> Vec<&K> {
    let mut keys = m.keys().collect::<Vec<&K>>();

    keys.sort();

    keys
}

#[test]
fn map() {
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].as_slice());
    let m = reader.deserialize::<HashMap<u8, u16>>().unwrap();
    assert!(m.is_empty());

    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x12, 0x67,
        ]
        .as_slice(),
    );
    let m = reader.deserialize::<HashMap<u8, u16>>().unwrap();
    assert_eq!(sorted_keys(&m), [&1]);
    assert_eq!(m.get(&1).unwrap(), &4711);

    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x12, 0x67, 0x02, 0x02, 0x9A,
        ]
        .as_slice(),
    );
    let m = reader.deserialize::<HashMap<u8, u16>>().unwrap();
    assert_eq!(sorted_keys(&m), [&1, &2]);
    assert_eq!(m.get(&1).unwrap(), &4711);
    assert_eq!(m.get(&2).unwrap(), &666);
}

#[test]
fn unit() {
    let mut reader = Reader::new([].as_slice());
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
    let mut reader = Reader::new([].as_slice());
    assert_eq!(reader.deserialize::<UnitStruct>().unwrap(), UnitStruct);

    // newtype-struct
    let mut reader = Reader::new([0x12, 0x67].as_slice());
    assert_eq!(
        reader.deserialize::<NewTypeStruct>().unwrap(),
        NewTypeStruct(4711)
    );

    // tuple-struct
    let mut reader = Reader::new([0x12, 0x67, 0x00, 0x00, 0x02, 0x9A].as_slice());
    assert_eq!(
        reader.deserialize::<TupleStruct>().unwrap(),
        TupleStruct(4711, 666)
    );

    // struct
    let mut reader = Reader::new([0x12, 0x67, 0x00, 0x00, 0x02, 0x9A].as_slice());
    assert_eq!(
        reader.deserialize::<Struct>().unwrap(),
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
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x00].as_slice());
    assert_eq!(reader.deserialize::<Enum>().unwrap(), Enum::V1);

    // newtype-variant
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x01, 0x12, 0x67].as_slice());
    assert_eq!(reader.deserialize::<Enum>().unwrap(), Enum::V2(4711));

    // tuple-variant
    let mut reader =
        Reader::new([0x00, 0x00, 0x00, 0x02, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A].as_slice());
    assert_eq!(reader.deserialize::<Enum>().unwrap(), Enum::V3(4711, 666));

    // struct-variant
    let mut reader =
        Reader::new([0x00, 0x00, 0x00, 0x03, 0x12, 0x67, 0x00, 0x00, 0x02, 0x9A].as_slice());
    assert_eq!(
        reader.deserialize::<Enum>().unwrap(),
        Enum::V4 { f1: 4711, f2: 666 }
    );

    // invalid index
    let mut reader = Reader::new([0x00, 0x00, 0x00, 0x04].as_slice());
    let err = reader.deserialize::<Enum>().unwrap_err();
    assert_error_eq!(
        err,
        Error::Serde(|msg| "invalid value: integer `4`, expected variant index 0 <= i < 4")
    );
}
