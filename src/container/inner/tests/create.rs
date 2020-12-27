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

use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::inner::Inner;
use crate::password::PasswordStore;
use crate::types::{Cipher, DiskType, Options};

fn setup(dtype: DiskType, bsize: u32) -> (TempDir, PathBuf, Options, PasswordStore) {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();
    let mut options = Options::default_with_cipher(Cipher::None).unwrap();
    let store = PasswordStore::new();

    options.set_dtype(dtype);
    options.set_bsize(bsize).unwrap();
    options.set_blocks(3).unwrap();

    (tmp_dir, path, options, store)
}

#[test]
fn thin_random_512() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::ThinRandom, 512);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
    assert_eq!(path.as_path().metadata().unwrap().len(), 512);
}

#[test]
fn thin_zero_512() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::ThinZero, 512);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
    assert_eq!(path.as_path().metadata().unwrap().len(), 512);
}

#[test]
fn fat_random_512() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::FatRandom, 512);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
    assert_eq!(path.as_path().metadata().unwrap().len(), 1536);
}

#[test]
fn fat_zero_512() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::FatZero, 512);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
    assert_eq!(path.as_path().metadata().unwrap().len(), 1536);
}

#[test]
fn thin_random_1024() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::ThinRandom, 1024);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
    assert_eq!(path.as_path().metadata().unwrap().len(), 1024);
}

#[test]
fn thin_zero_1024() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::ThinZero, 1024);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 1);
    assert_eq!(path.as_path().metadata().unwrap().len(), 1024);
}

#[test]
fn fat_random_1024() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::FatRandom, 1024);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
    assert_eq!(path.as_path().metadata().unwrap().len(), 3072);
}

#[test]
fn fat_zero_1024() {
    let (_tmp_dir, path, options, mut store) = setup(DiskType::FatZero, 1024);
    let inner = Inner::create(&path, &options, &mut store).unwrap();

    assert_eq!(inner.ablocks, 3);
    assert_eq!(path.as_path().metadata().unwrap().len(), 3072);
}
