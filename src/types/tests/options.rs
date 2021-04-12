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

use crate::error::Error;
use crate::rand::RND;
use crate::types::{Cipher, Digest, DiskType, Options, WrappingKey};

#[test]
fn default() {
    let options = Options::default().unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(
        options.wkey,
        Some(WrappingKey::Pbkdf2 {
            digest: Digest::Sha1,
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
            digest: Digest::Sha1,
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
fn set_dtype_fat_zero() {
    let mut options = Options::default().unwrap();

    options.set_dtype(DiskType::FatZero);
    assert_eq!(options.dtype, DiskType::FatZero);
}

#[test]
fn set_dtype_fat_random() {
    let mut options = Options::default().unwrap();

    options.set_dtype(DiskType::FatRandom);
    assert_eq!(options.dtype, DiskType::FatRandom);
}

#[test]
fn set_dtype_thin_zero() {
    let mut options = Options::default().unwrap();

    options.set_dtype(DiskType::ThinZero);
    assert_eq!(options.dtype, DiskType::ThinZero);
}

#[test]
fn set_dtype_thin_random() {
    let mut options = Options::default().unwrap();

    options.set_dtype(DiskType::ThinRandom);
    assert_eq!(options.dtype, DiskType::ThinRandom);
}

#[test]
fn set_wkey_none_cipher() {
    let mut options = Options::default_with_cipher(Cipher::None).unwrap();

    options.set_wkey(WrappingKey::generate_pbkdf2(Digest::Sha1, 1, 2).unwrap());
    assert_eq!(options.wkey, None);
}

#[test]
fn set_wkey_some_cipher() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    options.set_wkey(WrappingKey::generate_pbkdf2(Digest::Sha1, 1, 2).unwrap());
    if let Some(WrappingKey::Pbkdf2 {
        digest,
        iterations,
        salt,
    }) = options.wkey
    {
        assert_eq!(digest, Digest::Sha1);
        assert_eq!(iterations, 1);
        assert_eq!(salt.len(), 2);
    } else {
        panic!("invalid wrapping key");
    }
}

#[test]
fn set_digest_none_cipher() {
    let mut options = Options::default_with_cipher(Cipher::None).unwrap();

    options.set_digest(Digest::Sha1);
    assert_eq!(options.md, None);
}

#[test]
fn set_digest_some_cipher() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    options.set_digest(Digest::Sha1);
    assert_eq!(options.md, Some(Digest::Sha1));
}

#[test]
fn set_bsize_below_min() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    let err = options.set_bsize(511).unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "Invalid block size, got 511 but must be at least 512.");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn set_bsize_inval() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    let err = options.set_bsize(513).unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(
            msg,
            "Invalid block size, got 513 but must be a multiple of 512."
        );
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn set_blocks_inval() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    let err = options.set_blocks(0).unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "Invalid number of blocks, got 0, expected > 0.");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn set_size_below_min() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    options.set_size(511);
    assert_eq!(options.blocks, 1);
}

#[test]
fn set_size_inval() {
    let mut options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();

    options.set_size(513);
    assert_eq!(options.blocks, 1);

    options.set_size(1025);
    assert_eq!(options.blocks, 2);

    options.set_size(1537);
    assert_eq!(options.blocks, 3);
}
