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

use crate::bytes::error::Error;
use crate::bytes::options::Options;

#[test]
fn bool() {
    for n in [1, 2] {
        let b: bool = Options::new().from_bytes(&[n]).unwrap();
        assert_eq!(b, true);
    }

    let b: bool = Options::new().from_bytes(&[0]).unwrap();
    assert_eq!(b, false);
}

#[test]
fn u8() {
    for n in 0..2 {
        let o: u8 = Options::new().from_bytes(&[n]).unwrap();
        assert_eq!(o, n);
    }
}

#[test]
fn u16() {
    for (buf, n) in [([0], 0x00), ([1], 0x01), ([2], 0x02)] {
        let r: u16 = Options::new().from_bytes(&buf).unwrap();
        assert_eq!(r, n);
    }
}

#[test]
fn u32() {
    for (buf, n) in [([0], 0x00), ([1], 0x01), ([2], 0x02)] {
        let r: u32 = Options::new().from_bytes(&buf).unwrap();
        assert_eq!(r, n);
    }
}

#[test]
fn u64() {
    for (buf, n) in [([0], 0x00), ([1], 0x01), ([2], 0x02)] {
        let r: u64 = Options::new().from_bytes(&buf).unwrap();
        assert_eq!(r, n);
    }
}

#[test]
fn u128() {
    let err = Options::new().from_bytes::<u128>(&[0]).unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(msg, "u128 is not supported");
}

#[test]
fn char() {
    let c = Options::new()
        .with_fixint()
        .from_bytes::<char>(&[0x00, 0x01, 0xF4, 0xAF])
        .unwrap();
    assert_eq!(c, '💯');

    let err = Options::new()
        .with_fixint()
        .from_bytes::<char>(&[0x00, 0x11, 0x00, 0x00])
        .unwrap_err();
    let n = into_error!(err, Error::InvalidChar);
    assert_eq!(n, 0x110000);
}

#[test]
fn str() {
    for (bytes, str) in [
        (vec![0x00], ""),
        (vec![0x01, b'a'], "a"),
        (vec![0x02, b'a', b'b'], "ab"),
        (vec![0x03, b'a', b'b', b'c'], "abc"),
    ] {
        let r = Options::new().from_bytes::<&str>(&bytes).unwrap();
        assert_eq!(r, str);
    }

    let err = Options::new()
        .from_bytes::<&str>(&[0x04, 0, 159, 146, 150])
        .unwrap_err();
    let err = into_error!(err, Error::InvalidString);
    assert_eq!(
        err.to_string(),
        "invalid utf-8 sequence of 1 bytes from index 1"
    );
}

#[test]
fn string() {
    for (bytes, str) in [
        (vec![0x00], ""),
        (vec![0x01, b'a'], "a"),
        (vec![0x02, b'a', b'b'], "ab"),
        (vec![0x03, b'a', b'b', b'c'], "abc"),
    ] {
        let r = Options::new().from_bytes::<String>(&bytes).unwrap();
        assert_eq!(r, str);
    }

    let err = Options::new()
        .from_bytes::<String>(&[0x04, 0, 159, 146, 150])
        .unwrap_err();
    let err = into_error!(err, Error::InvalidString);
    assert_eq!(
        err.to_string(),
        "invalid utf-8 sequence of 1 bytes from index 1"
    );
}

#[test]
fn array() {
    let r: [u16; 0] = Options::new().from_bytes(&[]).unwrap();
    assert_eq!(r, []);

    let r: [u16; 1] = Options::new().from_bytes(&[1]).unwrap();
    assert_eq!(r, [1]);

    let r: [u16; 2] = Options::new().from_bytes(&[1, 2]).unwrap();
    assert_eq!(r, [1, 2]);

    let r: [u16; 3] = Options::new().from_bytes(&[1, 2, 3]).unwrap();
    assert_eq!(r, [1, 2, 3]);
}

#[test]
fn bytes() {
    for (buf, bytes) in [
        (vec![0x00], vec![]),
        (vec![0x01, 1], vec![1]),
        (vec![0x02, 1, 2], vec![1, 2]),
        (vec![0x03, 1, 2, 3], vec![1, 2, 3]),
    ] {
        let r = Options::new().from_bytes::<&[u8]>(&buf).unwrap();
        assert_eq!(r, bytes);
    }
}

#[test]
fn vec() {
    for (buf, bytes) in [
        (vec![0x00], vec![]),
        (vec![0x01, 1], vec![1]),
        (vec![0x02, 1, 2], vec![1, 2]),
        (vec![0x03, 1, 2, 3], vec![1, 2, 3]),
    ] {
        let r = Options::new().from_bytes::<Vec<u16>>(&buf).unwrap();
        assert_eq!(r, bytes);
    }
}

#[test]
fn option() {
    for buf in [[1, 1], [2, 1]] {
        let r: Option<u16> = Options::new().from_bytes(&buf).unwrap();
        assert_eq!(r, Some(1));
    }

    let r: Option<u16> = Options::new().from_bytes(&[0]).unwrap();
    assert_eq!(r, None);
}

fn sorted_keys<K: Ord, V>(m: &HashMap<K, V>) -> Vec<&K> {
    let mut keys = m.keys().collect::<Vec<&K>>();

    keys.sort();

    keys
}

#[test]
fn map() {
    let m = Options::new()
        .from_bytes::<HashMap<u8, u16>>(&[0x00])
        .unwrap();
    assert!(m.is_empty());

    let m = Options::new()
        .from_bytes::<HashMap<u8, u16>>(&[0x01, 0x01, 251, 0x12, 0x67])
        .unwrap();
    assert_eq!(sorted_keys(&m), [&1]);
    assert_eq!(m.get(&1).unwrap(), &4711);

    let m = Options::new()
        .from_bytes::<HashMap<u8, u16>>(&[0x02, 0x01, 251, 0x12, 0x67, 0x02, 251, 0x02, 0x9A])
        .unwrap();
    assert_eq!(sorted_keys(&m), [&1, &2]);
    assert_eq!(m.get(&1).unwrap(), &4711);
    assert_eq!(m.get(&2).unwrap(), &666);
}

#[test]
fn unit() {
    let u = Options::new().from_bytes::<()>(&[]).unwrap();
    assert_eq!(u, ());
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
    let s = Options::new().from_bytes::<UnitStruct>(&[]).unwrap();
    assert_eq!(s, UnitStruct);

    // newtype-struct
    let s = Options::new()
        .with_fixint()
        .from_bytes::<NewTypeStruct>(&[0x12, 0x67])
        .unwrap();
    assert_eq!(s, NewTypeStruct(4711));

    // tuple-struct
    let s = Options::new()
        .with_fixint()
        .from_bytes::<TupleStruct>(&[0x12, 0x67, 0x00, 0x00, 0x02, 0x9A])
        .unwrap();
    assert_eq!(s, TupleStruct(4711, 666));

    // struct
    let s = Options::new()
        .with_fixint()
        .from_bytes::<Struct>(&[0x12, 0x67, 0x00, 0x00, 0x02, 0x9A])
        .unwrap();
    assert_eq!(s, Struct { f1: 4711, f2: 666 });
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
    let e = Options::new().from_bytes::<Enum>(&[0x00]).unwrap();
    assert_eq!(e, Enum::V1);

    // newtype-variant
    let e = Options::new()
        .from_bytes::<Enum>(&[0x01, 251, 0x12, 0x67])
        .unwrap();
    assert_eq!(e, Enum::V2(4711));

    // tuple-variant
    let e = Options::new()
        .from_bytes::<Enum>(&[0x02, 251, 0x12, 0x67, 251, 0x02, 0x9A])
        .unwrap();
    assert_eq!(e, Enum::V3(4711, 666));

    // struct-variant
    let e = Options::new()
        .from_bytes::<Enum>(&[0x03, 251, 0x12, 0x67, 251, 0x02, 0x9A])
        .unwrap();
    assert_eq!(e, Enum::V4 { f1: 4711, f2: 666 });

    // invalid index
    let err = Options::new().from_bytes::<Enum>(&[0x04]).unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(
        msg,
        "invalid value: integer `4`, expected variant index 0 <= i < 4"
    );
}
