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

use std::io::Cursor;

use crate::io::IO;
use crate::types::DiskType;

fn setup(vec: Vec<u8>) -> IO {
    let mut c = Cursor::new(vec);
    IO::new(4, 1, DiskType::FatRandom, &mut c).unwrap()
}

#[test]
fn empty() {
    let io = setup(vec![]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 0);
    assert_eq!(io.dtype, DiskType::FatRandom);
}

#[test]
fn half() {
    let io = setup(vec![1, 2]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 0);
    assert_eq!(io.dtype, DiskType::FatRandom);
}

#[test]
fn one() {
    let io = setup(vec![1, 2, 3, 4]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 1);
    assert_eq!(io.dtype, DiskType::FatRandom);
}

#[test]
fn one_half() {
    let io = setup(vec![1, 2, 3, 4, 5, 6]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 1);
    assert_eq!(io.dtype, DiskType::FatRandom);
}

#[test]
fn two() {
    let io = setup(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 2);
    assert_eq!(io.dtype, DiskType::FatRandom);
}

#[test]
fn two_half() {
    let io = setup(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(io.bsize, 4);
    assert_eq!(io.blocks, 1);
    assert_eq!(io.ablocks, 2);
    assert_eq!(io.dtype, DiskType::FatRandom);
}
