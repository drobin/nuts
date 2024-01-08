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

use crate::error::Error;
use crate::reader::Reader;

#[test]
fn bool() {
    let mut reader = Reader::new([0, 1, 2].as_slice());

    assert_eq!(reader.read::<bool>().unwrap(), false);
    assert_eq!(reader.read::<bool>().unwrap(), true);
    assert_eq!(reader.read::<bool>().unwrap(), true);
}

#[test]
fn i8() {
    let mut reader = Reader::new([255, 0, 1].as_slice());

    assert_eq!(reader.read::<i8>().unwrap(), -1);
    assert_eq!(reader.read::<i8>().unwrap(), 0);
    assert_eq!(reader.read::<i8>().unwrap(), 1);
}

#[test]
fn i16() {
    let mut reader = Reader::new([255, 255, 0, 0, 1, 2].as_slice());

    assert_eq!(reader.read::<i16>().unwrap(), -1);
    assert_eq!(reader.read::<i16>().unwrap(), 0);
    assert_eq!(reader.read::<i16>().unwrap(), 0x0102);
}

#[test]
fn i32() {
    let mut reader = Reader::new([255, 255, 255, 255, 0, 0, 0, 0, 1, 2, 3, 4].as_slice());

    assert_eq!(reader.read::<i32>().unwrap(), -1);
    assert_eq!(reader.read::<i32>().unwrap(), 0);
    assert_eq!(reader.read::<i32>().unwrap(), 0x01020304);
}

#[test]
fn i64() {
    let mut reader = Reader::new(
        [
            255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        ]
        .as_slice(),
    );

    assert_eq!(reader.read::<i64>().unwrap(), -1);
    assert_eq!(reader.read::<i64>().unwrap(), 0);
    assert_eq!(reader.read::<i64>().unwrap(), 0x0102030405060708);
}

#[test]
fn u8() {
    let mut reader = Reader::new([1].as_slice());

    assert_eq!(reader.read::<u8>().unwrap(), 1);
}

#[test]
fn u16() {
    let mut reader = Reader::new([1, 2].as_slice());

    assert_eq!(reader.read::<u16>().unwrap(), 0x0102);
}

#[test]
fn u32() {
    let mut reader = Reader::new([1, 2, 3, 4].as_slice());

    assert_eq!(reader.read::<u32>().unwrap(), 0x01020304);
}

#[test]
fn u64() {
    let mut reader = Reader::new([1, 2, 3, 4, 5, 6, 7, 8].as_slice());

    assert_eq!(reader.read::<u64>().unwrap(), 0x0102030405060708);
}

#[test]
fn f32() {
    let mut reader = Reader::new([0x41, 0x48, 0x00, 0x00].as_slice());

    assert_eq!(reader.read::<f32>().unwrap(), 12.5);
}

#[test]
fn f64() {
    let mut reader = Reader::new([0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].as_slice());

    assert_eq!(reader.read::<f64>().unwrap(), 12.5);
}

#[test]
fn char() {
    let mut reader = Reader::new([0x00, 0x01, 0xF4, 0xAF, 0x00, 0x11, 0x00, 0x00].as_slice());

    assert_eq!(reader.read::<char>().unwrap(), '💯');

    let err = reader.read::<char>().unwrap_err();
    assert!(matches!(err, Error::InvalidChar(1114112)));
}

#[test]
fn usize() {
    let mut reader = Reader::new([1, 2, 3, 4, 5, 6, 7, 8].as_slice());

    assert_eq!(reader.read::<usize>().unwrap(), 0x0102030405060708);
}

#[test]
fn array_zero() {
    let mut reader = Reader::new([].as_slice());

    assert_eq!(reader.read::<[u16; 0]>().unwrap(), []);
    assert_eq!(reader.read::<[u16; 0]>().unwrap(), []);
}

#[test]
fn array_one() {
    let mut reader = Reader::new([1, 2, 3, 4, 5].as_slice());

    assert_eq!(reader.read::<[u16; 1]>().unwrap(), [0x0102]);
    assert_eq!(reader.read::<[u16; 1]>().unwrap(), [0x0304]);
    assert_eq!(reader.as_ref(), &[5]);
}

#[test]
fn array_two() {
    let vec = (1..12).collect::<Vec<u8>>();
    let mut reader = Reader::new(vec.as_slice());

    assert_eq!(reader.read::<[u16; 2]>().unwrap(), [0x0102, 0x0304]);
    assert_eq!(reader.read::<[u16; 2]>().unwrap(), [0x0506, 0x0708]);
    assert_eq!(reader.as_ref(), &[9, 10, 11]);
}

#[test]
fn array_three() {
    let vec = (1..18).collect::<Vec<u8>>();
    let mut reader = Reader::new(vec.as_slice());

    assert_eq!(reader.read::<[u16; 3]>().unwrap(), [0x0102, 0x0304, 0x0506]);
    assert_eq!(reader.read::<[u16; 3]>().unwrap(), [0x0708, 0x090a, 0x0b0c]);
    assert_eq!(reader.as_ref(), &[13, 14, 15, 16, 17]);
}

#[test]
fn vec_zero() {
    let mut reader = Reader::new([0; 8].as_slice());

    assert_eq!(reader.read::<Vec<u16>>().unwrap(), []);
}

#[test]
fn vec_one() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 1, 0, 1].as_slice());

    assert_eq!(reader.read::<Vec<u16>>().unwrap(), [1]);
}

#[test]
fn vec_two() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 2].as_slice());

    assert_eq!(reader.read::<Vec<u16>>().unwrap(), [1, 2]);
}

#[test]
fn vec_three() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 2, 0, 3].as_slice());

    assert_eq!(reader.read::<Vec<u16>>().unwrap(), [1, 2, 3]);
}

#[test]
fn string_zero() {
    let mut reader = Reader::new([0; 8].as_slice());

    assert_eq!(reader.read::<String>().unwrap(), "");
}

#[test]
fn string_one() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 1, b'a'].as_slice());

    assert_eq!(reader.read::<String>().unwrap(), "a");
}

#[test]
fn string_two() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 2, b'a', b'b'].as_slice());

    assert_eq!(reader.read::<String>().unwrap(), "ab");
}

#[test]
fn string_three() {
    let mut reader = Reader::new([0, 0, 0, 0, 0, 0, 0, 3, b'a', b'b', b'c'].as_slice());

    assert_eq!(reader.read::<String>().unwrap(), "abc");
}

#[test]
fn option() {
    let mut reader = Reader::new([0x00, 0x01, 0x00, 0x01, 0x02, 0x00, 0x01].as_slice());

    assert_eq!(reader.read::<Option<u16>>().unwrap(), None);
    assert_eq!(reader.read::<Option<u16>>().unwrap(), Some(1));
    assert_eq!(reader.read::<Option<u16>>().unwrap(), Some(1));
}

#[test]
fn unit() {
    let mut reader = Reader::new([].as_slice());

    assert_eq!(reader.read::<()>().unwrap(), ());
}
