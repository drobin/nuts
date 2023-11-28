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

#[cfg(feature = "derive")]
use nuts_bytes::{ToBytes, Writer};

#[cfg(feature = "derive")]
#[test]
fn unit_struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample;

    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Sample).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}

#[cfg(feature = "derive")]
#[test]
fn empty_struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample {}

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample {}).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}

#[cfg(feature = "derive")]
#[test]
fn empty_tuple_struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample();

    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Sample {}).unwrap(), 0);
    assert_eq!(writer.into_target(), []);
}

#[cfg(feature = "derive")]
#[test]
fn r#struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample {
        f1: u16,
        f2: u32,
    }

    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Sample { f1: 1, f2: 2 }).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 1, 0, 0, 0, 2]);
}

#[cfg(feature = "derive")]
#[test]
fn newtype_struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample(u16);

    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Sample(1)).unwrap(), 2);
    assert_eq!(writer.into_target(), [0, 1]);
}

#[cfg(feature = "derive")]
#[test]
fn tuple_struct() {
    #[derive(Debug, PartialEq, ToBytes)]
    struct Sample(u16, u32);

    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&Sample(1, 2)).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 1, 0, 0, 0, 2]);
}

// #[cfg(feature = "derive")]
// #[test]
// fn zero_variant_enum() {
//     #[derive(ToBytes)]
//     enum Sample {}
//
//     // error: zero-variant enums cannot be instantiated
// }

#[cfg(feature = "derive")]
#[test]
fn r#enum() {
    #[derive(Debug, PartialEq, ToBytes)]
    enum Sample {
        V0,
        V1 {},
        V2 { f1: u16 },
        V3 { f1: u16, f2: u32 },
        V4(),
        V5(u16),
        V6(u16, u32),
    }

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V0).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 0, 0, 0]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V1 {}).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 0, 0, 1]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V2 { f1: 1 }).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 0, 0, 2, 0, 1]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V3 { f1: 1, f2: 2 }).unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 3, 0, 1, 0, 0, 0, 2]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V4()).unwrap(), 4);
    assert_eq!(writer.into_target(), [0, 0, 0, 4]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V5(1)).unwrap(), 6);
    assert_eq!(writer.into_target(), [0, 0, 0, 5, 0, 1]);

    let mut writer = Writer::new(vec![]);
    assert_eq!(writer.write(&Sample::V6(1, 2)).unwrap(), 10);
    assert_eq!(writer.into_target(), [0, 0, 0, 6, 0, 1, 0, 0, 0, 2]);
}
