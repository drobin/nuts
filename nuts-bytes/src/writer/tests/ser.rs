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
use crate::options::Options;
use crate::{assert_error_eq, VecTarget, Writer};

fn setup_var_writer() -> Writer<VecTarget> {
    Options::new().build_writer(VecTarget::new(vec![]))
}

fn setup_fix_writer() -> Writer<VecTarget> {
    Options::new()
        .with_fixint()
        .build_writer(VecTarget::new(vec![]))
}

#[test]
fn bool() {
    for (t, buf) in [(true, [1]), (false, [0])] {
        let mut writer = setup_var_writer();
        assert_eq!(t.serialize(&mut writer).unwrap(), 1);
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u8() {
    for n in 0u8..2 {
        let mut writer = setup_var_writer();
        assert_eq!(n.serialize(&mut writer).unwrap(), 1);
        assert_eq!(writer.as_ref().as_ref(), [n]);
    }
}

#[test]
fn u16() {
    for (n, buf) in [(0x00u16, [0]), (0x01, [1]), (0x02, [2])] {
        let mut writer = setup_var_writer();
        assert_eq!(n.serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u32() {
    for (n, buf) in [(0x00u32, [0]), (0x01, [1]), (0x02, [2])] {
        let mut writer = setup_var_writer();
        assert_eq!(n.serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u64() {
    for (n, buf) in [(0x00u64, [0]), (0x01, [1]), (0x02, [2])] {
        let mut writer = setup_var_writer();
        assert_eq!(n.serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn u128() {
    let mut writer = setup_var_writer();
    let err = 0u128.serialize(&mut writer).unwrap_err();
    assert_error_eq!(err, Error::Serde(|msg| "u128 is not supported"));
}

#[test]
fn char() {
    let mut writer = setup_fix_writer();
    assert_eq!('💯'.serialize(&mut writer).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x00, 0x01, 0xF4, 0xAF]);
}

#[test]
fn str() {
    for (str, buf) in [
        ("", vec![0x00]),
        ("a", vec![0x01, b'a']),
        ("ab", vec![0x02, b'a', b'b']),
        ("abc", vec![0x03, b'a', b'b', b'c']),
    ] {
        let mut writer = setup_var_writer();
        assert_eq!(str.serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
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
        let mut writer = setup_var_writer();
        assert_eq!(str.to_string().serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn array() {
    let arr: [u16; 0] = [];
    let mut writer = setup_var_writer();
    assert_eq!(arr.serialize(&mut writer).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);

    let arr: [u16; 1] = [1];
    let mut writer = setup_var_writer();
    assert_eq!(arr.serialize(&mut writer).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [1]);

    let arr: [u16; 2] = [1, 2];
    let mut writer = setup_var_writer();
    assert_eq!(arr.serialize(&mut writer).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 2]);

    let arr: [u16; 3] = [1, 2, 3];
    let mut writer = setup_var_writer();
    assert_eq!(arr.serialize(&mut writer).unwrap(), 3);
    assert_eq!(writer.as_ref().as_ref(), [1, 2, 3]);
}

#[test]
fn bytes() {
    for (bytes, buf) in [
        (vec![], vec![0x00]),
        (vec![1u8], vec![0x01, 1]),
        (vec![1, 2], vec![0x02, 1, 2]),
        (vec![1, 2, 3], vec![0x03, 1, 2, 3]),
    ] {
        let mut writer = setup_var_writer();
        assert_eq!(bytes.as_slice().serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn vec() {
    for (bytes, buf) in [
        (vec![], vec![0x00]),
        (vec![1u16], vec![0x01, 1]),
        (vec![1, 2], vec![0x02, 1, 2]),
        (vec![1, 2, 3], vec![0x03, 1, 2, 3]),
    ] {
        let mut writer = setup_var_writer();
        assert_eq!(bytes.serialize(&mut writer).unwrap(), buf.len());
        assert_eq!(writer.as_ref().as_ref(), buf);
    }
}

#[test]
fn option() {
    let mut writer = setup_var_writer();
    assert_eq!(Some(1u16).serialize(&mut writer).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [1, 1]);

    let mut writer = setup_var_writer();
    assert_eq!(Option::<u16>::None.serialize(&mut writer).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0]);
}

#[test]
fn map() {
    let map = HashMap::<u8, u16>::new();
    let mut writer = setup_var_writer();
    assert_eq!(map.serialize(&mut writer).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0x00]);

    let map = HashMap::<u8, u16>::from([(1, 4711)]);
    let mut writer = setup_var_writer();
    assert_eq!(map.serialize(&mut writer).unwrap(), 5);
    assert_eq!(writer.as_ref().as_ref(), [0x01, 0x01, 251, 0x12, 0x67]);

    let map = HashMap::<u8, u16>::from([(1, 4711), (2, 666)]);
    let mut writer = setup_var_writer();
    assert_eq!(map.serialize(&mut writer).unwrap(), 9);
    assert_eq!(writer.as_ref().as_ref()[0], 0x02);

    if writer.as_ref().as_ref()[1] == 0x01 {
        assert_eq!(
            writer.as_ref().as_ref()[1..],
            [0x01, 251, 0x12, 0x67, 0x02, 251, 0x02, 0x9A]
        );
    } else {
        assert_eq!(
            writer.as_ref().as_ref()[1..],
            [0x02, 251, 0x02, 0x9A, 0x01, 251, 0x12, 0x67]
        );
    }
}

#[test]
fn unit() {
    let mut writer = setup_var_writer();
    assert_eq!(().serialize(&mut writer).unwrap(), 0);
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
    let mut writer = setup_var_writer();
    assert_eq!(UnitStruct.serialize(&mut writer).unwrap(), 0);
    assert_eq!(writer.as_ref().as_ref(), []);

    // newtype-struct
    let mut writer = setup_fix_writer();
    assert_eq!(NewTypeStruct(4711).serialize(&mut writer).unwrap(), 2);
    assert_eq!(writer.as_ref().as_ref(), [0x12, 0x67]);

    // tuple-struct
    let mut writer = setup_fix_writer();
    assert_eq!(TupleStruct(4711, 666).serialize(&mut writer).unwrap(), 6);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x12, 0x67, 0x00, 0x00, 0x02, 0x9A]
    );

    // struct
    let mut writer = setup_fix_writer();
    assert_eq!(
        Struct { f1: 4711, f2: 666 }.serialize(&mut writer).unwrap(),
        6
    );
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
    let mut writer = setup_var_writer();
    assert_eq!(Enum::V1.serialize(&mut writer).unwrap(), 1);
    assert_eq!(writer.as_ref().as_ref(), [0x00]);

    // newtype-variant
    let mut writer = setup_var_writer();
    assert_eq!(Enum::V2(4711).serialize(&mut writer).unwrap(), 4);
    assert_eq!(writer.as_ref().as_ref(), [0x01, 251, 0x12, 0x67]);

    // tuple-variant
    let mut writer = setup_var_writer();
    assert_eq!(Enum::V3(4711, 666).serialize(&mut writer).unwrap(), 7);
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x02, 251, 0x12, 0x67, 251, 0x02, 0x9A]
    );

    // struct-variant
    let mut writer = setup_var_writer();
    assert_eq!(
        Enum::V4 { f1: 4711, f2: 666 }
            .serialize(&mut writer)
            .unwrap(),
        7
    );
    assert_eq!(
        writer.as_ref().as_ref(),
        [0x03, 251, 0x12, 0x67, 251, 0x02, 0x9A]
    );
}
