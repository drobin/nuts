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

use crate::rand::RND;
use crate::types::{Cipher, Digest, DiskType, Options, WrappingKey};

#[test]
fn update_sites_ok() {
    let mut options = Options::default().unwrap();
    assert!(options.update_sizes(1024, 2).is_ok());
    assert_eq!(options.bsize, 1024);
    assert_eq!(options.blocks, 2);
}

#[test]
#[should_panic(expected = "Invalid block size, got 1 but must be at least 512.")]
fn update_sites_bsize_too_small() {
    let mut options = Options::default().unwrap();
    options.update_sizes(1, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid block size, got 513 but must be a multiple of 512.")]
fn update_sites_bsize_inval() {
    let mut options = Options::default().unwrap();
    options.update_sizes(513, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid number of blocks, got 0, expected > 0.")]
fn update_sites_blocks_inval() {
    let mut options = Options::default().unwrap();
    options.update_sizes(512, 0).unwrap();
}

#[test]
fn default() {
    let options = Options::default().unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        Some(WrappingKey::Pbkdf2 {
            iterations: 65536,
            salt: RND[..16].to_vec()
        })
    );
    assert_eq!(options.cipher, Cipher::Aes128Ctr);
    assert_eq!(options.md, Some(Digest::Sha1));
    assert_eq!(options.bsize, 512);
    assert_eq!(options.blocks, 2048);
}

#[test]
fn default_with_cipher_none() {
    let options = Options::default_with_cipher(Cipher::None).unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(options.wkey, None);
    assert_eq!(options.cipher, Cipher::None);
    assert_eq!(options.md, None);
    assert_eq!(options.bsize, 512);
    assert_eq!(options.blocks, 2048);
}

#[test]
fn default_with_cipher_aes128_ctr() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        Some(WrappingKey::Pbkdf2 {
            iterations: 65536,
            salt: RND[..16].to_vec(),
        })
    );
    assert_eq!(options.cipher, Cipher::Aes128Ctr);
    assert_eq!(options.md, Some(Digest::Sha1));
    assert_eq!(options.bsize, 512);
    assert_eq!(options.blocks, 2048);
}

#[test]
fn defaut_with_sizes_ok() {
    let options = Options::default_with_sizes(1024, 2).unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        Some(WrappingKey::Pbkdf2 {
            iterations: 65536,
            salt: RND[..16].to_vec()
        })
    );
    assert_eq!(options.cipher, Cipher::Aes128Ctr);
    assert_eq!(options.md, Some(Digest::Sha1));
    assert_eq!(options.bsize, 1024);
    assert_eq!(options.blocks, 2);
}

#[test]
#[should_panic(expected = "Invalid block size, got 4711 but must be a multiple of 512.")]
fn default_with_sizes_inval_bsize() {
    Options::default_with_sizes(4711, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid number of blocks, got 0, expected > 0.")]
fn default_with_sizes_inval_blocks() {
    Options::default_with_sizes(1024, 0).unwrap();
}
