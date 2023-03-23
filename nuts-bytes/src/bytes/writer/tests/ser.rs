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

use crate::bytes::error::Error;
use crate::bytes::into_error;
use crate::bytes::options::Options;

#[test]
fn bool() {
    let vec = Options::new().to_vec(&true).unwrap();
    assert_eq!(vec, [1]);

    let vec = Options::new().to_vec(&false).unwrap();
    assert_eq!(vec, [0]);
}

#[test]
fn u8() {
    for n in 0..2 {
        let vec = Options::new().to_vec::<u8>(&n).unwrap();
        assert_eq!(vec, [n]);
    }
}

#[test]
fn u16() {
    for (n, buf) in [(0x00, [0]), (0x01, [1]), (0x02, [2])] {
        let vec = Options::new().to_vec::<u16>(&n).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn u32() {
    for (n, buf) in [(0x00, [0]), (0x01, [1]), (0x02, [2])] {
        let vec = Options::new().to_vec::<u32>(&n).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn u64() {
    for (n, buf) in [(0x00, [0]), (0x01, [1]), (0x02, [2])] {
        let vec = Options::new().to_vec::<u64>(&n).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn u128() {
    let err = Options::new().to_vec::<u128>(&0).unwrap_err();
    let msg = into_error!(err, Error::Serde);
    assert_eq!(msg, "u128 is not supported");
}

#[test]
fn char() {
    let vec = Options::new().with_fixint().to_vec(&'💯').unwrap();
    assert_eq!(vec, [0x00, 0x01, 0xF4, 0xAF]);
}

#[test]
fn str() {
    for (str, buf) in [
        ("", vec![0x00]),
        ("a", vec![0x01, b'a']),
        ("ab", vec![0x02, b'a', b'b']),
        ("abc", vec![0x03, b'a', b'b', b'c']),
    ] {
        let vec = Options::new().to_vec(&str).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn string() {
    for (str, buf) in [
        ("", vec![0x00]),
        ("a", vec![0x01, b'a']),
        ("ab", vec![0x02, b'a', b'b']),
        ("abc", vec![0x03, b'a', b'b', b'c']),
    ] {
        let vec = Options::new().to_vec(&str.to_string()).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn array() {
    let vec = Options::new().to_vec::<[u16; 0]>(&[]).unwrap();
    assert_eq!(vec, []);

    let vec = Options::new().to_vec::<[u16; 1]>(&[1]).unwrap();
    assert_eq!(vec, [1]);

    let vec = Options::new().to_vec::<[u16; 2]>(&[1, 2]).unwrap();
    assert_eq!(vec, [1, 2]);

    let vec = Options::new().to_vec::<[u16; 3]>(&[1, 2, 3]).unwrap();
    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn bytes() {
    for (bytes, buf) in [
        (vec![], vec![0x00]),
        (vec![1], vec![0x01, 1]),
        (vec![1, 2], vec![0x02, 1, 2]),
        (vec![1, 2, 3], vec![0x03, 1, 2, 3]),
    ] {
        let vec = Options::new().to_vec::<&[u8]>(&bytes.as_slice()).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn vec() {
    for (bytes, buf) in [
        (vec![], vec![0x00]),
        (vec![1], vec![0x01, 1]),
        (vec![1, 2], vec![0x02, 1, 2]),
        (vec![1, 2, 3], vec![0x03, 1, 2, 3]),
    ] {
        let vec = Options::new().to_vec::<Vec<u16>>(&bytes).unwrap();
        assert_eq!(vec, buf);
    }
}

#[test]
fn option() {
    let vec = Options::new().to_vec::<Option<u16>>(&Some(1)).unwrap();
    assert_eq!(vec, [1, 1]);

    let vec = Options::new().to_vec::<Option<u16>>(&None).unwrap();
    assert_eq!(vec, [0]);
}

#[test]
fn map() {
    let map = HashMap::<u8, u16>::new();
    let vec = Options::new().to_vec(&map).unwrap();
    assert_eq!(vec, [0x00]);

    let map = HashMap::<u8, u16>::from([(1, 4711)]);
    let vec = Options::new().to_vec(&map).unwrap();
    assert_eq!(vec, [0x01, 0x01, 251, 0x12, 0x67]);

    let map = HashMap::<u8, u16>::from([(1, 4711), (2, 666)]);
    let vec = Options::new().to_vec(&map).unwrap();
    assert_eq!(vec[0], 0x02);

    if vec[1] == 0x01 {
        assert_eq!(vec[1..], [0x01, 251, 0x12, 0x67, 0x02, 251, 0x02, 0x9A]);
    } else {
        assert_eq!(vec[1..], [0x02, 251, 0x02, 0x9A, 0x01, 251, 0x12, 0x67]);
    }
}

#[test]
fn unit() {
    let vec = Options::new().to_vec(&()).unwrap();
    assert_eq!(vec, []);
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
    let vec = Options::new().to_vec(&UnitStruct).unwrap();
    assert_eq!(vec, []);

    // newtype-struct
    let vec = Options::new()
        .with_fixint()
        .to_vec(&NewTypeStruct(4711))
        .unwrap();
    assert_eq!(vec, [0x12, 0x67]);

    // tuple-struct
    let vec = Options::new()
        .with_fixint()
        .to_vec(&TupleStruct(4711, 666))
        .unwrap();
    assert_eq!(vec, [0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);

    // struct
    let vec = Options::new()
        .with_fixint()
        .to_vec(&Struct { f1: 4711, f2: 666 })
        .unwrap();
    assert_eq!(vec, [0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]);
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
    let vec = Options::new().to_vec(&Enum::V1).unwrap();
    assert_eq!(vec, [0x00]);

    // newtype-variant
    let vec = Options::new().to_vec(&Enum::V2(4711)).unwrap();
    assert_eq!(vec, [0x01, 251, 0x12, 0x67]);

    // tuple-variant
    let vec = Options::new().to_vec(&Enum::V3(4711, 666)).unwrap();
    assert_eq!(vec, [0x02, 251, 0x12, 0x67, 251, 0x02, 0x9A]);

    // struct-variant
    let vec = Options::new()
        .to_vec(&Enum::V4 { f1: 4711, f2: 666 })
        .unwrap();
    assert_eq!(vec, [0x03, 251, 0x12, 0x67, 251, 0x02, 0x9A]);
}
