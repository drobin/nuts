// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use std::io::{Cursor, ErrorKind};

use crate::io::{FromBinary, IntoBinary};
use crate::types::DiskType;

#[test]
fn from_binary_fat_zero() {
    let mut c = Cursor::new(&[0]);
    assert_eq!(DiskType::from_binary(&mut c).unwrap(), DiskType::FatZero);
}

#[test]
fn from_binary_fat_random() {
    let mut c = Cursor::new(&[1]);
    assert_eq!(DiskType::from_binary(&mut c).unwrap(), DiskType::FatRandom);
}

#[test]
fn from_binary_thin_zero() {
    let mut c = Cursor::new(&[2]);
    assert_eq!(DiskType::from_binary(&mut c).unwrap(), DiskType::ThinZero);
}

#[test]
fn from_binary_thin_random() {
    let mut c = Cursor::new(&[3]);
    assert_eq!(DiskType::from_binary(&mut c).unwrap(), DiskType::ThinRandom);
}

#[test]
fn from_binary_inval() {
    let mut c = Cursor::new(&[4]);
    let err = DiskType::from_binary(&mut c).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(format!("{}", err), "invalid disk-type detected");
}

#[test]
fn into_binary_fat_zero() {
    let mut c = Cursor::new(Vec::new());

    DiskType::FatZero.into_binary(&mut c).unwrap();
    assert_eq!(c.into_inner(), [0]);
}

#[test]
fn into_binary_fat_random() {
    let mut c = Cursor::new(Vec::new());

    DiskType::FatRandom.into_binary(&mut c).unwrap();
    assert_eq!(c.into_inner(), [1]);
}

#[test]
fn into_binary_thin_zero() {
    let mut c = Cursor::new(Vec::new());

    DiskType::ThinZero.into_binary(&mut c).unwrap();
    assert_eq!(c.into_inner(), [2]);
}

#[test]
fn into_binary_thin_random() {
    let mut c = Cursor::new(Vec::new());

    DiskType::ThinRandom.into_binary(&mut c).unwrap();
    assert_eq!(c.into_inner(), [3]);
}
