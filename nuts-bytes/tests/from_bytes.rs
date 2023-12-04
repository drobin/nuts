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

#![cfg(feature = "derive")]

mod common;

use nuts_bytes::{Error, FromBytes, Reader};

use crate::common::{
    assert_sample_error, map_eq_mod, map_err_from_bytes, map_err_mod, map_from_bytes, map_mod,
};

#[test]
fn unit_struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample;

    let mut reader = Reader::new([].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(sample, Sample);
}

#[test]
fn empty_struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample {}

    let mut reader = Reader::new([].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(sample, Sample {});
}

#[test]
fn empty_tuple_struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample();

    let mut reader = Reader::new([].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(sample, Sample {});
}

#[test]
fn r#struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample {
        f1: u16,
        f2: u32,
        #[nuts_bytes(map = map_eq_mod)]
        f3: u8,
        #[nuts_bytes(map = map_mod)]
        f4: u16,
        #[nuts_bytes(map_from_bytes = map_from_bytes)]
        f5: u16,
        #[nuts_bytes(map = map_mod, map_from_bytes = map_from_bytes)]
        f6: u16,
        #[nuts_bytes(map_from_bytes = map_from_bytes, map = map_mod)]
        f7: u16,
    }

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2, 3, 4, 5, 6, 7].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(
        sample,
        Sample {
            f1: 1,
            f2: 2,
            f3: 3,
            f4: 5,
            f5: 7,
            f6: 8,
            f7: 9
        }
    );
}

#[test]
fn r#struct_map_err() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample {
        f1: u16,
        #[nuts_bytes(map = map_err_mod)]
        f2: u32,
    }

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2].as_slice());

    let err = reader.read::<Sample>().unwrap_err();
    assert_sample_error!(err);
}

#[test]
fn r#struct_map_from_bytes_err() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample {
        f1: u16,
        #[nuts_bytes(map_from_bytes = map_err_from_bytes)]
        f2: u32,
    }

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2].as_slice());

    let err = reader.read::<Sample>().unwrap_err();
    assert_sample_error!(err);
}

#[test]
fn newtype_struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample(u16);

    let mut reader = Reader::new([0, 1].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(sample, Sample(1));
}

#[test]
fn tuple_struct() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample(
        u16,
        u32,
        #[nuts_bytes(map = map_eq_mod)] u8,
        #[nuts_bytes(map = map_mod)] u16,
        #[nuts_bytes(map_from_bytes = map_from_bytes)] u16,
        #[nuts_bytes(map = map_mod, map_from_bytes = map_from_bytes)] u16,
        #[nuts_bytes(map_from_bytes = map_from_bytes, map = map_mod)] u16,
    );

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2, 3, 4, 5, 6, 7].as_slice());
    let sample = reader.read::<Sample>().unwrap();

    assert_eq!(sample, Sample(1, 2, 3, 5, 7, 8, 9));
}

#[test]
fn tuple_struct_map_err() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample(u16, #[nuts_bytes(map = map_err_mod)] u32);

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2].as_slice());

    let err = reader.read::<Sample>().unwrap_err();
    assert_sample_error!(err);
}

#[test]
fn tuple_struct_map_from_bytes_err() {
    #[derive(Debug, FromBytes, PartialEq)]
    struct Sample(u16, #[nuts_bytes(map_from_bytes = map_err_from_bytes)] u32);

    let mut reader = Reader::new([0, 1, 0, 0, 0, 2].as_slice());

    let err = reader.read::<Sample>().unwrap_err();
    assert_sample_error!(err);
}

// #[test]
// fn zero_variant_enum() {
//     #[derive(FromBytes)]
//     enum Sample {}
//
//     // error: zero-variant enums cannot be instantiated
// }

#[test]
fn r#enum() {
    #[derive(Debug, FromBytes, PartialEq)]
    enum Sample {
        V0,
        V1 {},
        V2 { f1: u16 },
        V3 { f1: u16, f2: u32 },
        V4(),
        V5(u16),
        V6(u16, u32),
    }

    let mut reader = Reader::new([0, 0, 0, 0].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V0);

    let mut reader = Reader::new([0, 0, 0, 1].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V1 {});

    let mut reader = Reader::new([0, 0, 0, 2, 0, 1].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V2 { f1: 1 });

    let mut reader = Reader::new([0, 0, 0, 3, 0, 1, 0, 0, 0, 2].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V3 { f1: 1, f2: 2 });

    let mut reader = Reader::new([0, 0, 0, 4].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V4());

    let mut reader = Reader::new([0, 0, 0, 5, 0, 1].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V5(1));

    let mut reader = Reader::new([0, 0, 0, 6, 0, 1, 0, 0, 0, 2].as_slice());
    let sample = reader.read::<Sample>().unwrap();
    assert_eq!(sample, Sample::V6(1, 2));

    let mut reader = Reader::new([0, 0, 0, 7].as_slice());
    let err = reader.read::<Sample>().unwrap_err();
    assert!(matches!(err, Error::InvalidVariantIndex(7)));
}
