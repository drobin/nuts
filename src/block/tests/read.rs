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

use crate::block::Block;
use crate::error::Error;

fn mk_fake_file() -> std::io::Cursor<Vec<u8>> {
    let mut c = Cursor::new(vec![1, 2, 3, 4, 5, 6]);
    c.seek(SeekFrom::Start(0)).unwrap();
    c
}

fn prepare(bsize: Option<u32>) -> (Block, Cursor<Vec<u8>>, [u8; 3]) {
    let block = Block::build(bsize.unwrap_or(3));
    let source = mk_fake_file();
    let target = [0; 3];

    (block, source, target)
}

#[test]
fn full_block_0() {
    let (block, mut source, mut target) = prepare(None);

    assert_eq!(block.read(&mut source, &mut target, 0).unwrap(), 3);
    assert_eq!(target, [1, 2, 3]);
    assert_eq!(source.position(), 3);
}

#[test]
fn full_block_1() {
    let (block, mut source, mut target) = prepare(None);

    assert_eq!(block.read(&mut source, &mut target, 1).unwrap(), 3);
    assert_eq!(target, [4, 5, 6]);
    assert_eq!(source.position(), 6);
}

#[test]
fn part_block_0() {
    let (block, mut source, mut target) = prepare(None);
    let buf = target.get_mut(0..2).unwrap();

    assert_eq!(block.read(&mut source, buf, 0).unwrap(), 2);
    assert_eq!(target, [1, 2, 0]);
    assert_eq!(source.position(), 2);
}

#[test]
fn part_block_1() {
    let (block, mut source, mut target) = prepare(None);
    let buf = target.get_mut(0..2).unwrap();

    assert_eq!(block.read(&mut source, buf, 1).unwrap(), 2);
    assert_eq!(target, [4, 5, 0]);
    assert_eq!(source.position(), 5);
}

#[test]
fn bsize_0_block_0() {
    let (block, mut source, mut target) = prepare(Some(0));

    assert_eq!(block.read(&mut source, &mut target, 0).unwrap(), 0);
    assert_eq!(target, [0, 0, 0]);
    assert_eq!(source.position(), 0);
}

#[test]
fn bsize_0_block_1() {
    let (block, mut source, mut target) = prepare(Some(0));

    assert_eq!(block.read(&mut source, &mut target, 1).unwrap(), 0);
    assert_eq!(target, [0, 0, 0]);
    assert_eq!(source.position(), 0);
}

#[test]
fn bsize_2_block_0() {
    let (block, mut source, mut target) = prepare(Some(2));

    assert_eq!(block.read(&mut source, &mut target, 0).unwrap(), 2);
    assert_eq!(target, [1, 2, 0]);
    assert_eq!(source.position(), 2);
}

#[test]
fn bsize_2_block_1() {
    let (block, mut source, mut target) = prepare(Some(2));

    assert_eq!(block.read(&mut source, &mut target, 1).unwrap(), 2);
    assert_eq!(target, [3, 4, 0]);
    assert_eq!(source.position(), 4);
}

#[test]
fn seek_into_last_block() {
    let (block, mut source, mut target) = prepare(Some(4));

    if let Error::IoError(err) = block.read(&mut source, &mut target, 1).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn seek_behind_last_id() {
    let (block, mut source, mut target) = prepare(None);

    if let Error::IoError(err) = block.read(&mut source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}
