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

use std::io::ErrorKind;

use crate::error::InvalHeaderKind;
use crate::header::Header;
use crate::types::{Cipher, Digest, DiskType, WrappingKeyData, BLOCK_MIN_SIZE};

const ENCODED_SIZE: u32 = 155;
const ENCODED_WKEY_DATA: [u8; 12] = [1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3];
const ENCODED_WRAPPING_IV: [u8; 20] = [
    0x00, 0x00, 0x00, 0x10, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
];
const ENCODED_HMAC: [u8; 24] = [
    0, 0, 0, 20, 208, 98, 13, 189, 168, 244, 235, 189, 4, 29, 124, 252, 76, 223, 98, 90, 100, 202,
    104, 26,
];
const ENCODED_SECRET: [u8; 89] = [
    0, 0, 0, 85, 21, 41, 218, 239, 228, 245, 41, 150, 29, 113, 119, 117, 150, 178, 29, 147, 144,
    100, 134, 111, 47, 5, 92, 46, 136, 34, 229, 149, 229, 214, 30, 226, 197, 251, 52, 53, 192, 49,
    150, 111, 85, 161, 122, 173, 223, 205, 185, 225, 78, 217, 224, 146, 31, 186, 146, 196, 199,
    222, 232, 79, 170, 98, 176, 179, 202, 46, 0, 142, 172, 167, 183, 51, 21, 62, 115, 101, 214,
    190, 72, 53, 163, 199, 77, 238, 42,
];

fn ok_header() -> Header {
    Header {
        revision: 1,
        cipher: Cipher::Aes128Ctr,
        digest: Some(Digest::Sha1),
        wrapping_key_data: Some(WrappingKeyData::Pbkdf2 {
            iterations: 4711,
            salt: vec![1, 2, 3],
        }),
        wrapping_iv: vec![
            13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
        ],
        dtype: DiskType::FatRandom,
        bsize: BLOCK_MIN_SIZE,
        blocks: 4711,
        master_key: secure_vec![b'a'; 16],
        master_iv: secure_vec![b'b'; 16],
        hmac_key: secure_vec![b'c'; 20],
        userdata: vec![7, 8, 9, 10],
    }
}

fn setup() -> (Header, [u8; 256]) {
    (ok_header(), [0; 256])
}

#[test]
fn ok() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 1); // cipher
    assert_eq!(target[9], 1); // digest
    assert_eq!(target[10..22], ENCODED_WKEY_DATA); // pbkdf2
    assert_eq!(target[22..42], ENCODED_WRAPPING_IV); // wrapping_iv
    assert_eq!(target[42..66], ENCODED_HMAC); // hmac
    assert_eq!(target[66..98], ENCODED_SECRET[..32]); // secret, part I
    assert_eq!(&target[98..130], &ENCODED_SECRET[32..64]); // secret, part II
    assert_eq!(&target[130..155], &ENCODED_SECRET[64..]); // secret, part III
}

#[test]
fn no_space() {
    let (header, mut target) = setup();

    for i in 1..ENCODED_SIZE as usize {
        assert_io_error!(
            ErrorKind::WriteZero,
            header.write(&mut target.get_mut(..i).unwrap(), b"123")
        );
    }
}

#[test]
fn digest_none() {
    let (mut header, mut target) = setup();
    header.digest = None;

    assert_inval_header!(
        InvalHeaderKind::InvalDigest,
        header.write(&mut target, b"123")
    );
}

#[test]
fn digest_sha1() {
    let (mut header, mut target) = setup();
    header.digest = Some(Digest::Sha1);

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
    assert_eq!(target[9], 1);
}

#[test]
fn wrapping_key_data_none() {
    let (mut header, mut target) = setup();
    header.wrapping_key_data = None;

    assert_inval_header!(
        InvalHeaderKind::InvalWrappingKey,
        header.write(&mut target, b"123")
    );
}

#[test]
fn wrapping_key_data_pbkdf2() {
    let (mut header, mut target) = setup();
    header.wrapping_key_data = Some(WrappingKeyData::Pbkdf2 {
        iterations: 4711,
        salt: vec![1, 2, 3],
    });

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
    assert_eq!(target[10..22], ENCODED_WKEY_DATA);
}

#[test]
fn wrapping_iv_inval_size() {
    let (mut header, mut target) = setup();
    header.wrapping_iv.pop().unwrap();

    assert_inval_header!(InvalHeaderKind::InvalIv, header.write(&mut target, b"123"));
}

#[test]
fn wrapping_iv() {
    let (header, mut target) = setup();

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
    assert_eq!(target[22..42], ENCODED_WRAPPING_IV);
}

#[test]
fn bsize_lt_512() {
    let (mut header, mut target) = setup();
    header.bsize = BLOCK_MIN_SIZE - 1;

    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        header.write(&mut target, b"123")
    );
}

#[test]
fn bsize_inval_modulo() {
    let (mut header, mut target) = setup();
    header.bsize = BLOCK_MIN_SIZE + 1;

    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        header.write(&mut target, b"123")
    );
}

#[test]
fn bsize_512() {
    let (mut header, mut target) = setup();
    header.bsize = 512;

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
}

#[test]
fn bsize_1024() {
    let (mut header, mut target) = setup();
    header.bsize = 1024;

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_0() {
    let (mut header, mut target) = setup();
    header.blocks = 0;

    assert_inval_header!(
        InvalHeaderKind::InvalBlocks,
        header.write(&mut target, b"123")
    );
}

#[test]
fn blocks_1() {
    let (mut header, mut target) = setup();
    header.blocks = 1;

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_2() {
    let (mut header, mut target) = setup();
    header.blocks = 2;

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE);
}

#[test]
fn master_key_inval_size() {
    let (mut header, mut target) = setup();
    header.master_key.pop().unwrap();

    assert_inval_header!(
        InvalHeaderKind::InvalMasterKey,
        header.write(&mut target, b"123")
    );
}

#[test]
fn master_iv_inval_size() {
    let (mut header, mut target) = setup();
    header.master_iv.pop().unwrap();

    assert_inval_header!(
        InvalHeaderKind::InvalMasterIv,
        header.write(&mut target, b"123")
    );
}

#[test]
fn hmac_key_inval_size() {
    let (mut header, mut target) = setup();
    header.hmac_key.pop().unwrap();

    assert_inval_header!(
        InvalHeaderKind::InvalHmacKey,
        header.write(&mut target, b"123")
    );
}

#[test]
fn empty_userdata() {
    let (mut header, mut target) = setup();
    header.userdata.clear();

    assert_eq!(header.write(&mut target, b"123").unwrap(), ENCODED_SIZE - 4);
}
