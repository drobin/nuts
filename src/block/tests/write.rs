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

fn mk_fake_file() -> std::io::Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

fn prepare(bsize: Option<u32>) -> (Block, [u8; 6], Cursor<Vec<u8>>) {
    let block = Block::build(bsize.unwrap_or(3));
    let source = [1, 2, 3, 4, 5, 6];
    let target = mk_fake_file();

    (block, source, target)
}

#[test]
fn full_block_0() {
    let (block, mut source, mut target) = prepare(None);

    assert_eq!(block.write(&mut source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3]);
}

#[test]
fn full_block_1() {
    let (block, mut source, mut target) = prepare(None);

    assert_eq!(block.write(&mut source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 3]);
}

#[test]
fn part_block_0() {
    let (block, mut source, mut target) = prepare(None);
    let buf = source.get_mut(0..2).unwrap();

    assert_eq!(block.write(buf, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 0]);
}

#[test]
fn part_block_1() {
    let (block, mut source, mut target) = prepare(None);
    let buf = source.get_mut(0..2).unwrap();

    assert_eq!(block.write(buf, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 0]);
}

#[test]
fn more_block_0() {
    let (block, mut source, mut target) = prepare(Some(2));

    assert_eq!(block.write(&mut source, &mut target, 0).unwrap(), 2);
    assert_eq!(target.into_inner(), [1, 2]);
}

#[test]
fn more_block_1() {
    let (block, mut source, mut target) = prepare(Some(2));

    assert_eq!(block.write(&mut source, &mut target, 1).unwrap(), 2);
    assert_eq!(target.into_inner(), [0, 0, 1, 2]);
}
