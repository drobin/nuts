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

use crate::openssl::RND;
use crate::secret::Secret;
use crate::types::{Cipher, DiskType, Options, BLOCK_MIN_SIZE};

#[test]
fn cipher_none() {
    let options = Options::default_with_cipher(Cipher::None);
    let secret = Secret::create(&options).unwrap();

    assert_eq!(secret.dtype, DiskType::FatRandom);
    assert_eq!(secret.bsize, BLOCK_MIN_SIZE);
    assert_eq!(secret.blocks, 2048);
    assert!(secret.master_key.is_empty());
    assert!(secret.master_iv.is_empty());
    assert!(secret.hmac_key.is_empty());
    assert!(secret.userdata.is_empty());
}

#[test]
fn cipher_aes128_ctr() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr);
    let secret = Secret::create(&options).unwrap();

    assert_eq!(secret.dtype, DiskType::FatRandom);
    assert_eq!(secret.bsize, BLOCK_MIN_SIZE);
    assert_eq!(secret.blocks, 2048);
    assert_eq!(secret.master_key, &RND[..16]);
    assert_eq!(secret.master_iv, &RND[..16]);
    assert_eq!(secret.hmac_key, &RND[..20]);
    assert!(secret.userdata.is_empty());
}
