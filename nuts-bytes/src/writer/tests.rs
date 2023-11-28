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

use crate::put_bytes::PutBytesError;
use crate::to_bytes::ToBytesError;
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
fn bool_nospace() {
    let mut buf = [b'x'; 0];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&true).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
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
fn i8_nospace() {
    let mut buf = [b'x'; 0];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1i8).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
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
fn i16_nospace() {
    let mut buf = [b'x'; 1];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1i16).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x']);
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
fn i32_nospace() {
    let mut buf = [b'x'; 3];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1i32).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 3]);
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
fn i64_nospace() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1i64).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn u8() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u8).unwrap(), 1);
    assert_eq!(writer.into_target(), [1]);
}

#[test]
fn u8_nospace() {
    let mut buf = [b'x'; 0];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1u8).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
}

#[test]
fn u16() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u16).unwrap(), 2);
    assert_eq!(writer.into_target(), [0, 1]);
}

#[test]
fn u16_nospace() {
    let mut buf = [b'x'; 1];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1u16).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x']);
}

#[test]
fn u32() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u32).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 0, 0, 1]);
}

#[test]
fn u32_nospace() {
    let mut buf = [b'x'; 3];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1u32).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 3]);
}

#[test]
fn u64() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1u64).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn u64_nospace() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1u64).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn f32() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&12.5f32).unwrap(), 4);
    assert_eq!(writer.into_target(), [0x41, 0x48, 0x00, 0x00]);
}

#[test]
fn f32_nospace() {
    let mut buf = [b'x'; 3];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&12.5f32).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 3]);
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
fn f64_nospace() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&12.5f64).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn usize() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&1usize).unwrap(), 8);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn usize_nospace() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&1usize).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn char() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&'💯').unwrap(), 4);
    assert_eq!(writer.into_target(), [0x00, 0x01, 0xF4, 0xAF]);
}

#[test]
fn char_nospace() {
    let mut buf = [b'x'; 3];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&'💯').unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 3]);
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
fn array_one_nospace() {
    let mut buf = [b'x'; 1];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x']);
}

#[test]
fn array_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16]).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 1, 0, 2]);
}

#[test]
fn array_two_nospace() {
    let mut buf = [b'x'; 3];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 1, b'x']);
}

#[test]
fn array_three() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16, 3u16]).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 1, 0, 2, 0, 3]);
}

#[test]
fn array_three_nospace() {
    let mut buf = [b'x'; 5];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16, 3u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 1, 0, 2, b'x']);
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
fn slice_one_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn slice_one_nospace_value() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 1, b'x']);
}

#[test]
fn slice_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&[1u16, 2u16].as_slice()).unwrap(), 12);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 2]);
}

#[test]
fn slice_two_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn slice_two_nospace_value_1() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 2, b'x']);
}

#[test]
fn slice_two_nospace_value_2() {
    let mut buf = [b'x'; 11];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, b'x']);
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
fn slice_three_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16, 3u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn slice_three_nospace_value_1() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16, 3u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, b'x']);
}

#[test]
fn slice_three_nospace_value_2() {
    let mut buf = [b'x'; 11];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16, 3u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, b'x']);
}

#[test]
fn slice_three_nospace_value_3() {
    let mut buf = [b'x'; 13];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&[1u16, 2u16, 3u16].as_slice()).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 2, b'x']);
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
fn vec_one_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn vec_one_nospace_value() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 1, b'x']);
}

#[test]
fn vec_two() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&vec![1u16, 2u16]).unwrap(), 12);
    assert_eq!(writer.into_target(), [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 2]);
}

#[test]
fn vec_two_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn vec_two_nospace_value_1() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 2, b'x']);
}

#[test]
fn vec_two_nospace_value_2() {
    let mut buf = [b'x'; 11];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 2, 0, 1, b'x']);
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
fn vec_three_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16, 3u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn vec_three_nospace_value_1() {
    let mut buf = [b'x'; 9];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16, 3u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, b'x']);
}

#[test]
fn vec_three_nospace_value_2() {
    let mut buf = [b'x'; 11];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16, 3u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, b'x']);
}

#[test]
fn vec_three_nospace_value_3() {
    let mut buf = [b'x'; 13];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&vec![1u16, 2u16, 3u16]).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 2, b'x']);
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
fn str_nospace_len() {
    let mut buf = [b'x'; 7];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&"123").unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 7]);
}

#[test]
fn str_nospace_value() {
    let mut buf = [b'x'; 10];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&"123").unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [0, 0, 0, 0, 0, 0, 0, 3, b'1', b'2']);
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
fn option_none_nospace() {
    let mut buf = [b'x'; 0];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&None::<Option<u16>>).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [b'x'; 0]);
}

#[test]
fn option_some_nospace() {
    let mut buf = [b'x'; 2];
    let mut writer = Writer::new(&mut buf[..]);

    let err = writer.write(&Some(1u16)).unwrap_err();
    assert!(matches!(
        err,
        ToBytesError::PutBytes(PutBytesError::NoSpace)
    ));
    assert_eq!(buf, [1, b'x']);
}

#[test]
fn unit() {
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&()).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}
