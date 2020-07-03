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

use std::io::{Cursor, ErrorKind, Seek, SeekFrom};

use crate::error::Error;
use crate::io::IO;
use crate::types::DiskType;

fn mk_fake_file() -> std::io::Cursor<Vec<u8>> {
    let mut c = Cursor::new(vec![1, 2, 3, 4, 5, 6]);
    c.seek(SeekFrom::Start(0)).unwrap();
    c
}

fn prepare(bsize: u32, blocks: u64, tlen: usize) -> (IO, Cursor<Vec<u8>>, Vec<u8>) {
    let mut source = mk_fake_file();
    let io = IO::new(bsize, blocks, DiskType::ThinZero, &mut source).unwrap();

    (io, source, vec![b'x'; tlen])
}

#[test]
fn allocated_0_full() {
    let (io, mut source, mut target) = prepare(3, 2, 3);

    assert_eq!(io.read(&mut source, &mut target, 0).unwrap(), 3);
    assert_eq!(target, [1, 2, 3]);
}

#[test]
fn allocated_1_full() {
    let (io, mut source, mut target) = prepare(3, 2, 3);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 3);
    assert_eq!(target, [4, 5, 6]);
}

#[test]
fn allocated_0_part() {
    let (io, mut source, mut target) = prepare(3, 2, 2);

    assert_eq!(io.read(&mut source, &mut target, 0).unwrap(), 2);
    assert_eq!(target, [1, 2]);
}

#[test]
fn allocated_1_part() {
    let (io, mut source, mut target) = prepare(3, 2, 2);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 2);
    assert_eq!(target, [4, 5]);
}

#[test]
fn allocated_0_bigger() {
    let (io, mut source, mut target) = prepare(3, 2, 4);

    assert_eq!(io.read(&mut source, &mut target, 0).unwrap(), 3);
    assert_eq!(target, [1, 2, 3, b'x']);
}

#[test]
fn allocated_1_bigger() {
    let (io, mut source, mut target) = prepare(3, 2, 4);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 3);
    assert_eq!(target, [4, 5, 6, b'x']);
}

#[test]
fn unallocated_1_full() {
    let (io, mut source, mut target) = prepare(6, 3, 6);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 6);
    assert_eq!(target, [0; 6]);
}

#[test]
fn unallocated_2_full() {
    let (io, mut source, mut target) = prepare(6, 3, 6);

    assert_eq!(io.read(&mut source, &mut target, 2).unwrap(), 6);
    assert_eq!(target, [0; 6]);
}

#[test]
fn unallocated_1_part() {
    let (io, mut source, mut target) = prepare(6, 3, 5);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 5);
    assert_eq!(target, [0; 5]);
}

#[test]
fn unallocated_2_part() {
    let (io, mut source, mut target) = prepare(6, 3, 5);

    assert_eq!(io.read(&mut source, &mut target, 2).unwrap(), 5);
    assert_eq!(target, [0; 5]);
}

#[test]
fn unallocated_1_bigger() {
    let (io, mut source, mut target) = prepare(6, 3, 7);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 6);
    assert_eq!(target, [0, 0, 0, 0, 0, 0, b'x']);
}

#[test]
fn unallocated_2_bigger() {
    let (io, mut source, mut target) = prepare(6, 3, 7);

    assert_eq!(io.read(&mut source, &mut target, 2).unwrap(), 6);
    assert_eq!(target, [0, 0, 0, 0, 0, 0, b'x']);
}

#[test]
fn bsize_0_block_0() {
    let (io, mut source, mut target) = prepare(0, 2, 3);

    assert_eq!(io.read(&mut source, &mut target, 0).unwrap(), 0);
    assert_eq!(target, [b'x'; 3]);
}

#[test]
fn bsize_0_block_1() {
    let (io, mut source, mut target) = prepare(0, 2, 3);

    assert_eq!(io.read(&mut source, &mut target, 1).unwrap(), 0);
    assert_eq!(target, [b'x'; 3]);
}

#[test]
fn no_such_block() {
    let (io, mut source, mut target) = prepare(3, 2, 3);

    if let Error::IoError(err) = io.read(&mut source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}
