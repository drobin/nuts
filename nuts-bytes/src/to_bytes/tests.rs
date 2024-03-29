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

use crate::writer::Writer;

#[test]
fn bool_true() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&true).unwrap(), 1);
    assert_eq!(writer.into_target(), [1]);
}

#[test]
fn bool_false() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&false).unwrap(), 1);
    assert_eq!(writer.into_target(), [0]);
}

#[test]
fn i8() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&-1i8).unwrap(), 1);
    assert_eq!(writer.write(&0i8).unwrap(), 1);
    assert_eq!(writer.write(&1i8).unwrap(), 1);
    assert_eq!(writer.into_target(), [255, 0, 1]);
}

#[test]
fn i16() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&-1i16).unwrap(), 2);
    assert_eq!(writer.write(&0i16).unwrap(), 2);
    assert_eq!(writer.write(&1i16).unwrap(), 2);
    assert_eq!(writer.into_target(), [255, 255, 0, 0, 0, 1]);
}

#[test]
fn i32() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&-1i32).unwrap(), 4);
    assert_eq!(writer.write(&0i32).unwrap(), 4);
    assert_eq!(writer.write(&1i32).unwrap(), 4);
    assert_eq!(
        writer.into_target(),
        [255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 1]
    );
}

#[test]
fn i64() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&-1i64).unwrap(), 8);
    assert_eq!(writer.write(&0i64).unwrap(), 8);
    assert_eq!(writer.write(&1i64).unwrap(), 8);
    assert_eq!(
        writer.into_target(),
        [255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
}

#[test]
fn u8() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u8).unwrap(), 1);
    assert_eq!(writer.into_target(), [1]);
}

#[test]
fn u16() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u16).unwrap(), 2);
    assert_eq!(writer.into_target(), [0, 1]);
}

#[test]
fn u32() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u32).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 0, 0, 1]);
}

#[test]
fn u64() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u64).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn f32() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&12.5f32).unwrap(), 4);
    assert_eq!(writer.into_target(), [0x41, 0x48, 0x00, 0x00]);
}

#[test]
fn f64() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&12.5f64).unwrap(), 8);
    assert_eq!(
        writer.into_target(),
        [0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
}

#[test]
fn usize() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1usize).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn char() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&'💯').unwrap(), 4);
    assert_eq!(writer.into_target(), [0x00, 0x01, 0xF4, 0xAF]);
}

#[test]
fn array_zero() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[0u16; 0]).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}

#[test]
fn array_one() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16]).unwrap(), 2);
    assert_eq!(writer.into_target(), [0, 1]);
}

#[test]
fn array_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16]).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 1, 0, 2]);
}

#[test]
fn array_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16, 3u16]).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 1, 0, 2, 0, 3]);
}

#[test]
fn slice_zero() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[0u16; 0].as_slice()).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn slice_one() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16].as_slice()).unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1, 0, 1]);
}

#[test]
fn slice_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16].as_slice()).unwrap(), 12);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 2]);
}

#[test]
fn slice_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16, 3u16].as_slice()).unwrap(), 14);
    assert_eq!(
        writer.into_target(),
        [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 2, 0, 3]
    );
}

#[test]
fn vec_zero() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Vec::<u16>::new()).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn vec_one() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&vec![1u16]).unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1, 0, 1]);
}

#[test]
fn vec_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&vec![1u16, 2u16]).unwrap(), 12);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 2]);
}

#[test]
fn vec_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&vec![1u16, 2u16, 3u16]).unwrap(), 14);
    assert_eq!(
        writer.into_target(),
        [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 2, 0, 3]
    );
}

#[test]
fn str_zero() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&"").unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn str_one() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&"1").unwrap(), 9);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1, b'1']);
}

#[test]
fn str_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&"12").unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, b'1', b'2']);
}

#[test]
fn str_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&"123").unwrap(), 11);
    assert_eq!(
        writer.into_target(),
        [0, 0, 0, 0, 0, 0, 0, 3, b'1', b'2', b'3']
    );
}

#[test]
fn string_zero() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&String::from("")).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn string_one() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&String::from("1")).unwrap(), 9);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1, b'1']);
}

#[test]
fn string_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&String::from("12")).unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, b'1', b'2']);
}

#[test]
fn string_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&String::from("123")).unwrap(), 11);
    assert_eq!(
        writer.into_target(),
        [0, 0, 0, 0, 0, 0, 0, 3, b'1', b'2', b'3']
    );
}

#[test]
fn option_none() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&None::<Option<u16>>).unwrap(), 1);
    assert_eq!(writer.into_target(), [0x00]);
}

#[test]
fn option_some() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Some(1u16)).unwrap(), 3);
    assert_eq!(writer.into_target(), [0x01, 0x00, 0x01]);
}

#[test]
fn unit() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&()).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}
