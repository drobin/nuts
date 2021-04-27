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

use std::io::Read;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::Container;
use crate::io::Reader;
use crate::rand::RND;
use crate::types::{Cipher, OptionsBuilder, BLOCK_MIN_SIZE};

fn setup() -> (TempDir, Container) {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();
    let options = OptionsBuilder::new(Cipher::None)
        .with_bsize(BLOCK_MIN_SIZE)
        .with_blocks(4)
        .build()
        .unwrap();

    let mut container = Container::new();
    container.create(path, options).unwrap();

    assert_eq!(container.write(0, &RND[..512]).unwrap(), 512);
    assert_eq!(container.write(1, &RND[512..1024]).unwrap(), 512);
    assert_eq!(container.write(2, &RND[1024..1536]).unwrap(), 512);

    (tmp_dir, container)
}

#[test]
fn no_blocks() {
    let (_tmp_dir, mut container) = setup();
    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 16];

    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [0; 16]);
}

#[test]
fn one_block() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 512];

    reader.push_id(0);

    assert_eq!(reader.read(&mut buf).unwrap(), 512);
    assert_large_array!(buf, RND[..512]);
}

#[test]
fn one_block_larger_buf() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1024];

    reader.push_id(0);

    assert_eq!(reader.read(&mut buf).unwrap(), 512);
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..], [0; 512]);
}

#[test]
fn one_block_chunked_aligned() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 512];

    reader.push_id(0);

    assert_eq!(reader.read(&mut buf[..256]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf[256..]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_large_array!(buf, RND[..512]);
}

#[test]
fn one_block_chunked_unaligned() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1024];

    reader.push_id(0);

    assert_eq!(reader.read(&mut buf[..200]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[200..400]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[400..]).unwrap(), 112);
    assert_eq!(reader.read(&mut buf[512..]).unwrap(), 0);
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..], [0; 512]);
}

#[test]
fn two_blocks() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1024];

    reader.push_id(0);
    reader.push_id(1);

    assert_eq!(reader.read(&mut buf).unwrap(), 512);
    assert_eq!(reader.read(&mut buf[512..]).unwrap(), 512);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_large_array!(buf, RND[..1024]);
}

#[test]
fn two_blocks_larger_buf() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1536];

    reader.push_id(0);
    reader.push_id(1);

    assert_eq!(reader.read(&mut buf).unwrap(), 512);
    assert_eq!(reader.read(&mut buf[512..]).unwrap(), 512);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_large_array!(buf[..1024], RND[..1024]);
    assert_large_array!(buf[1024..], [0; 512]);
}

#[test]
fn two_blocks_chunked_aligned() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1024];

    reader.push_id(0);
    reader.push_id(1);

    assert_eq!(reader.read(&mut buf[..256]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf[256..512]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf[512..768]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf[768..1024]).unwrap(), 256);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_large_array!(buf, RND[..1024]);
}

#[test]
fn two_blocks_chunked_unaligned() {
    let (_tmp_dir, mut container) = setup();

    let mut reader = Reader::new(&mut container);
    let mut buf = [0; 1536];

    reader.push_id(0);
    reader.push_id(1);

    assert_eq!(reader.read(&mut buf[..200]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[200..400]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[400..600]).unwrap(), 112);
    assert_eq!(reader.read(&mut buf[512..712]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[712..912]).unwrap(), 200);
    assert_eq!(reader.read(&mut buf[912..]).unwrap(), 112);
    assert_eq!(reader.read(&mut buf[1024..]).unwrap(), 0);
    assert_large_array!(buf[..1024], RND[..1024]);
    assert_large_array!(buf[1024..], [0; 512]);
}
