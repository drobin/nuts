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
use crate::types::{Cipher, Digest, DiskType, OptionsBuilder, WrappingKey};

#[test]
fn default() {
    let options = OptionsBuilder::default().build().unwrap();
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
fn new_none() {
    let options = OptionsBuilder::new(Cipher::None).build().unwrap();
    assert_eq!(options.dtype, DiskType::FatRandom);
    assert_eq!(options.wkey, None);
    assert_eq!(options.cipher, Cipher::None);
    assert_eq!(options.md, None);
    assert_eq!(options.bsize, 512);
    assert_eq!(options.blocks, 2048);
}

#[test]
fn new_cipher_aes128_ctr() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr).build().unwrap();
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
fn with_dtype_fat_zero() {
    let options = OptionsBuilder::default()
        .with_dtype(DiskType::FatZero)
        .build()
        .unwrap();

    assert_eq!(options.dtype, DiskType::FatZero);
}

#[test]
fn with_dtype_fat_random() {
    let options = OptionsBuilder::default()
        .with_dtype(DiskType::FatRandom)
        .build()
        .unwrap();

    assert_eq!(options.dtype, DiskType::FatRandom);
}

#[test]
fn with_dtype_thin_zero() {
    let options = OptionsBuilder::default()
        .with_dtype(DiskType::ThinZero)
        .build()
        .unwrap();

    assert_eq!(options.dtype, DiskType::ThinZero);
}

#[test]
fn with_dtype_thin_random() {
    let options = OptionsBuilder::default()
        .with_dtype(DiskType::ThinRandom)
        .build()
        .unwrap();

    assert_eq!(options.dtype, DiskType::ThinRandom);
}

#[test]
fn with_wkey_none_cipher() {
    let options = OptionsBuilder::new(Cipher::None)
        .with_wkey(WrappingKey::generate_pbkdf2(Digest::Sha1, 1, 2).unwrap())
        .build()
        .unwrap();

    assert_eq!(options.wkey, None);
}

#[test]
fn with_wkey_some_cipher() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_wkey(WrappingKey::generate_pbkdf2(Digest::Sha1, 1, 2).unwrap())
        .build()
        .unwrap();

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
fn with_digest_none_cipher() {
    let options = OptionsBuilder::new(Cipher::None)
        .with_digest(Digest::Sha1)
        .build()
        .unwrap();

    assert_eq!(options.md, None);
}

#[test]
fn with_digest_some_cipher() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_digest(Digest::Sha1)
        .build()
        .unwrap();

    assert_eq!(options.md, Some(Digest::Sha1));
}

#[test]
fn with_bsize_below_min() {
    let err = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_bsize(511)
        .build()
        .unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "Invalid block size, got 511 but must be at least 512.");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn with_bsize_inval() {
    let err = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_bsize(513)
        .build()
        .unwrap_err();

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
fn with_blocks_inval() {
    let err = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_blocks(0)
        .build()
        .unwrap_err();

    if let Error::InvalArg(msg) = err {
        assert_eq!(msg, "Invalid number of blocks, got 0, expected > 0.");
    } else {
        panic!("invalid error: {:?}", err);
    }
}

#[test]
fn with_size_below_min() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_size(511)
        .build()
        .unwrap();

    assert_eq!(options.blocks, 1);
}

#[test]
fn with_size_inval_513() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_size(513)
        .build()
        .unwrap();
    assert_eq!(options.blocks, 1);
}

#[test]
fn with_size_inval_1025() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_size(1025)
        .build()
        .unwrap();
    assert_eq!(options.blocks, 2);
}

#[test]
fn with_size_inval_1537() {
    let options = OptionsBuilder::new(Cipher::Aes128Ctr)
        .with_size(1537)
        .build()
        .unwrap();
    assert_eq!(options.blocks, 3);
}
