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

use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::Container;
use crate::io::Writer;
use crate::rand::RND;
use crate::result::Result;
use crate::types::{Cipher, DiskType, Options, BLOCK_MIN_SIZE};

fn setup() -> (TempDir, Container) {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();
    let mut options = Options::default_with_cipher(Cipher::None).unwrap();

    options.set_dtype(DiskType::FatZero);
    options.set_bsize(BLOCK_MIN_SIZE).unwrap();
    options.set_blocks(4).unwrap();

    let mut container = Container::new();
    container.create(path, &options).unwrap();

    (tmp_dir, container)
}

fn read_container(container: &mut Container) -> Result<Vec<u8>> {
    let mut buf = vec![0; 1536];

    container.read(0, &mut buf[..512])?;
    container.read(1, &mut buf[512..1024])?;
    container.read(2, &mut buf[1024..1536])?;

    Ok(buf)
}

#[test]
fn no_blocks() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    let err = writer.write(&RND[..512]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Other);
    assert_eq!(format!("{}", err), "no more blocks available");
}

#[test]
fn one_block() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..512]).unwrap(), 512);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..], [0; 1024]);
}

#[test]
fn one_block_chunked() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..256]).unwrap(), 256);
    assert_eq!(writer.write(&RND[256..512]).unwrap(), 256);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..], [0; 1024]);
}

#[test]
fn two_blocks() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..512]).unwrap(), 512);
    assert_eq!(writer.write(&RND[512..1024]).unwrap(), 512);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..1024], RND[..1024]);
    assert_large_array!(buf[1024..], [0; 512]);
}

#[test]
fn two_blocks_chunked_aligned() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..256]).unwrap(), 256);
    assert_eq!(writer.write(&RND[256..512]).unwrap(), 256);
    assert_eq!(writer.write(&RND[512..768]).unwrap(), 256);
    assert_eq!(writer.write(&RND[768..1024]).unwrap(), 256);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..1024], RND[..1024]);
    assert_large_array!(buf[1024..], [0; 512]);
}

#[test]
fn two_blocks_chunked_aligned_padded() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..256]).unwrap(), 256);
    assert_eq!(writer.write(&RND[256..512]).unwrap(), 256);
    assert_eq!(writer.write(&RND[512..612]).unwrap(), 100);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..612], RND[512..612]);
    assert_large_array!(buf[612..], [0; 924]);
}

#[test]
fn two_blocks_chunked_unaligned() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..200]).unwrap(), 200);
    assert_eq!(writer.write(&RND[200..400]).unwrap(), 200);
    assert_eq!(writer.write(&RND[400..600]).unwrap(), 200);
    assert_eq!(writer.write(&RND[600..800]).unwrap(), 200);
    assert_eq!(writer.write(&RND[800..1000]).unwrap(), 200);
    assert_eq!(writer.write(&RND[1000..1024]).unwrap(), 24);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..1024], RND[..1024]);
    assert_large_array!(buf[1024..], [0; 512]);
}

#[test]
fn two_blocks_chunked_unaligned_padded() {
    let (_tmp_dir, mut container) = setup();

    let mut writer = Writer::new(&mut container);

    writer.push_id(0);
    writer.push_id(1);
    writer.push_id(2);

    assert_eq!(writer.write(&RND[..200]).unwrap(), 200);
    assert_eq!(writer.write(&RND[200..400]).unwrap(), 200);
    assert_eq!(writer.write(&RND[400..600]).unwrap(), 200);
    writer.flush().unwrap();

    let buf = read_container(&mut container).unwrap();
    assert_large_array!(buf[..512], RND[..512]);
    assert_large_array!(buf[512..600], RND[512..600]);
    assert_large_array!(buf[600..], [0; 936]);
}
