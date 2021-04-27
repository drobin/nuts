// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

use std::io::Cursor;

use crate::container::inner::Inner;
use crate::password::PasswordStore;
use crate::types::{Cipher, DiskType, OptionsBuilder};

fn setup(dtype: DiskType, bsize: u32) -> (Cursor<Vec<u8>>, PasswordStore) {
    let cursor = Cursor::new(vec![]);
    let mut store = PasswordStore::new();
    let options = OptionsBuilder::new(Cipher::None)
        .with_dtype(dtype)
        .with_bsize(bsize)
        .with_blocks(3)
        .build()
        .unwrap();

    let inner = Inner::create(cursor, options, &mut store).unwrap();
    let buf = inner.as_ref().get_ref().to_vec();

    (Cursor::new(buf), store)
}

#[test]
fn thin_random_512() {
    let (cursor, mut store) = setup(DiskType::ThinRandom, 512);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
}

#[test]
fn thin_zero_512() {
    let (cursor, mut store) = setup(DiskType::ThinZero, 512);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
}

#[test]
fn fat_random_512() {
    let (cursor, mut store) = setup(DiskType::FatRandom, 512);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
}

#[test]
fn fat_zero_512() {
    let (cursor, mut store) = setup(DiskType::FatZero, 512);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
}

#[test]
fn thin_random_1024() {
    let (cursor, mut store) = setup(DiskType::ThinRandom, 1024);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
}

#[test]
fn thin_zero_1024() {
    let (cursor, mut store) = setup(DiskType::ThinZero, 1024);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
}

#[test]
fn fat_random_1024() {
    let (cursor, mut store) = setup(DiskType::FatRandom, 1024);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
}

#[test]
fn fat_zero_1024() {
    let (cursor, mut store) = setup(DiskType::FatZero, 1024);
    let inner = Inner::open(cursor, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
}
