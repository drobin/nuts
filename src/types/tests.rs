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

use crate::types::{Cipher, Digest, DiskType, Options, WrappingKey};

#[test]
fn cipher_key_size_none() {
    assert_eq!(Cipher::None.key_size(), 0);
}

#[test]
fn cipher_key_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.key_size(), 16);
}

#[test]
fn cipher_iv_size_none() {
    assert_eq!(Cipher::None.iv_size(), 0);
}

#[test]
fn cipher_iv_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.iv_size(), 16);
}

#[test]
fn cipher_block_size_none() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn cipher_block_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn digest_size_sha1() {
    assert_eq!(Digest::Sha1.size(), 20);
}

#[test]
fn options_update_sites_ok() {
    let mut options = Options::default();
    assert!(options.update_sizes(1024, 2).is_ok());
    assert_eq!(options.bsize, 1024);
    assert_eq!(options.blocks, 2);
}

#[test]
#[should_panic(expected = "Invalid block size, got 1 but must be at least 512.")]
fn options_update_sites_bsize_too_small() {
    let mut options = Options::default();
    options.update_sizes(1, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid block size, got 513 but must be a multiple of 512.")]
fn options_update_sites_bsize_inval() {
    let mut options = Options::default();
    options.update_sizes(513, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid number of blocks, got 0, expected > 0.")]
fn options_update_sites_blocks_inval() {
    let mut options = Options::default();
    options.update_sizes(512, 0).unwrap();
}

#[test]
fn options_default() {
    let options = Options::default();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        WrappingKey::Pbkdf2 {
            iterations: 65536,
            salt_len: 16
        }
    );
    assert_eq!(options.cipher, Cipher::Aes128Ctr);
    assert_eq!(options.md, Digest::Sha1);
    assert_eq!(options.bsize, 512);
    assert_eq!(options.blocks, 2048);
}

#[test]
fn options_defaut_with_sizes_ok() {
    let options = Options::default_with_sizes(1024, 2).unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        WrappingKey::Pbkdf2 {
            iterations: 65536,
            salt_len: 16
        }
    );
    assert_eq!(options.cipher, Cipher::Aes128Ctr);
    assert_eq!(options.md, Digest::Sha1);
    assert_eq!(options.bsize, 1024);
    assert_eq!(options.blocks, 2);
}

#[test]
#[should_panic(expected = "Invalid block size, got 4711 but must be a multiple of 512.")]
fn options_default_with_sizes_inval_bsize() {
    Options::default_with_sizes(4711, 2).unwrap();
}

#[test]
#[should_panic(expected = "Invalid number of blocks, got 0, expected > 0.")]
fn options_default_with_sizes_inval_blocks() {
    Options::default_with_sizes(1024, 0).unwrap();
}
