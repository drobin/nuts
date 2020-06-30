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

use crate::secret::Secret;
use crate::types::{Cipher, Digest, DiskType, BLOCK_MIN_SIZE};

fn ok_secret() -> Secret {
    Secret {
        dtype: DiskType::FatRandom,
        bsize: BLOCK_MIN_SIZE,
        blocks: 4711,
        master_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        master_iv: vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ],
        hmac_key: vec![
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
        ],
        userdata: vec![7, 8, 9, 10],
    }
}

#[test]
fn cipher_none_digest_some() {
    let secret = ok_secret();
    let err = secret
        .validate(Cipher::None, Some(Digest::Sha1))
        .unwrap_err();

    assert_eq!(
        format!("{:?}", err),
        "InvalArg(\"digest cannot be sha1 for cipher none\")"
    );
}

#[test]
fn cipher_some_digest_none() {
    let secret = ok_secret();
    let err = secret.validate(Cipher::Aes128Ctr, None).unwrap_err();

    assert_eq!(
        format!("{:?}", err),
        "InvalArg(\"digest cannot be None for cipher aes128-ctr\")"
    );
}

#[test]
fn bsize_lt_512() {
    let mut secret = ok_secret();
    secret.bsize = BLOCK_MIN_SIZE - 1;

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalBlockSize)");
}

#[test]
fn bsize_inval_modulo() {
    let mut secret = ok_secret();
    secret.bsize = BLOCK_MIN_SIZE + 1;

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalBlockSize)");
}

#[test]
fn blocks_0() {
    let mut secret = ok_secret();
    secret.blocks = 0;

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalBlocks)");
}

#[test]
fn cipher_some_master_key_lt_key_size() {
    let mut secret = ok_secret();
    secret.master_key.pop();

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalMasterKey)");
}

#[test]
fn cipher_some_master_iv_lt_iv_size() {
    let mut secret = ok_secret();
    secret.master_iv.pop();

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalMasterIv)");
}

#[test]
fn bsize_512() {
    let mut secret = ok_secret();
    secret.bsize = 512;

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}

#[test]
fn bsize_1024() {
    let mut secret = ok_secret();
    secret.bsize = 1024;

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}

#[test]
fn blocks_1() {
    let mut secret = ok_secret();
    secret.blocks = 1;

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}

#[test]
fn blocks_2() {
    let mut secret = ok_secret();
    secret.blocks = 2;

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}

#[test]
fn digest_some_hmac_key_lt_digest_size() {
    let mut secret = ok_secret();
    secret.hmac_key.pop();

    let err = secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalHmacKey)");
}

#[test]
fn digest_some_hmac_key_eq_digest_size() {
    let secret = ok_secret();

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}

#[test]
fn cipher_none_keys_ignored() {
    let secret = ok_secret();
    secret.validate(Cipher::None, None).unwrap();
}

#[test]
fn cipher_none_empty_keys() {
    let mut secret = ok_secret();
    secret.master_key.clear();
    secret.master_iv.clear();
    secret.hmac_key.clear();

    secret.validate(Cipher::None, None).unwrap();
}

#[test]
fn cipher_some_keys_accepted() {
    let secret = ok_secret();

    secret
        .validate(Cipher::Aes128Ctr, Some(Digest::Sha1))
        .unwrap();
}
