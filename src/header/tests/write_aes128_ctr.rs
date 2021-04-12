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

use std::io::ErrorKind;

use crate::error::Error;
use crate::header::Header;
use crate::password::PasswordStore;
use crate::types::{Cipher, Digest, DiskType, WrappingKey, BLOCK_MIN_SIZE};

const ENCODED_SIZE: u32 = 107;
const ENCODED_WKEY_DATA: [u8; 13] = [
    1, 0x01, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
];
const ENCODED_WRAPPING_IV: [u8; 20] = [
    0x00, 0x00, 0x00, 0x10, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
];
const ENCODED_SECRET: [u8; 65] = [
    0, 0, 0, 61, 21, 41, 218, 239, 228, 245, 41, 150, 29, 113, 119, 117, 150, 178, 29, 147, 144,
    100, 134, 111, 47, 5, 92, 46, 136, 34, 229, 149, 229, 214, 30, 226, 197, 251, 52, 53, 192, 49,
    150, 111, 85, 161, 122, 173, 223, 205, 185, 225, 78, 217, 224, 146, 31, 186, 146, 196, 215,
    186, 131, 37, 195,
];

fn ok_header() -> Header {
    Header {
        revision: 1,
        cipher: Cipher::Aes128Ctr,
        wrapping_key: Some(WrappingKey::Pbkdf2 {
            digest: Digest::Sha1,
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
        userdata: vec![7, 8, 9, 10],
    }
}

fn setup(password: bool) -> (Header, [u8; 256], PasswordStore) {
    let mut store = PasswordStore::new();

    if password {
        store.set_value(secure_vec![b'1', b'2', b'3']);
    }

    (ok_header(), [0; 256], store)
}

#[test]
fn ok() {
    let (header, mut target, mut store) = setup(true);

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 1); // cipher
    assert_eq!(target[9..22], ENCODED_WKEY_DATA); // pbkdf2
    assert_eq!(target[22..42], ENCODED_WRAPPING_IV); // wrapping_iv
    assert_eq!(target[42..74], ENCODED_SECRET[..32]); // secret, part I
    assert_eq!(target[74..106], ENCODED_SECRET[32..64]); // secret, part II
    assert_eq!(&target[106..107], &ENCODED_SECRET[64..]); // secret, part III
}

#[test]
fn missing_callback() {
    let (header, mut target, mut store) = setup(false);

    assert_error!(Error::NoPassword, header.write(&mut target, &mut store));
}

#[test]
fn no_space() {
    let (header, mut target, mut store) = setup(true);

    for i in 1..ENCODED_SIZE as usize {
        assert_io_error!(
            ErrorKind::WriteZero,
            header.write(&mut target.get_mut(..i).unwrap(), &mut store)
        );
    }
}

#[test]
fn wrapping_key_data_none() {
    let (mut header, mut target, mut store) = setup(true);
    header.wrapping_key = None;

    assert_inval_header!("wrapping-key", header.write(&mut target, &mut store));
}

#[test]
fn wrapping_key_data_pbkdf2() {
    let (mut header, mut target, mut store) = setup(true);
    header.wrapping_key = Some(WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 4711,
        salt: vec![1, 2, 3],
    });

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[9..22], ENCODED_WKEY_DATA);
}

#[test]
fn wrapping_iv_inval_size() {
    let (mut header, mut target, mut store) = setup(true);
    header.wrapping_iv.pop().unwrap();

    assert_inval_header!("iv", header.write(&mut target, &mut store));
}

#[test]
fn wrapping_iv() {
    let (header, mut target, mut store) = setup(true);

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[22..42], ENCODED_WRAPPING_IV);
}

#[test]
fn bsize_lt_512() {
    let (mut header, mut target, mut store) = setup(true);
    header.bsize = BLOCK_MIN_SIZE - 1;

    assert_inval_header!("block-size", header.write(&mut target, &mut store));
}

#[test]
fn bsize_inval_modulo() {
    let (mut header, mut target, mut store) = setup(true);
    header.bsize = BLOCK_MIN_SIZE + 1;

    assert_inval_header!("block-size", header.write(&mut target, &mut store));
}

#[test]
fn bsize_512() {
    let (mut header, mut target, mut store) = setup(true);
    header.bsize = 512;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn bsize_1024() {
    let (mut header, mut target, mut store) = setup(true);
    header.bsize = 1024;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_0() {
    let (mut header, mut target, mut store) = setup(true);
    header.blocks = 0;

    assert_inval_header!("blocks", header.write(&mut target, &mut store));
}

#[test]
fn blocks_1() {
    let (mut header, mut target, mut store) = setup(true);
    header.blocks = 1;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_2() {
    let (mut header, mut target, mut store) = setup(true);
    header.blocks = 2;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn master_key_inval_size() {
    let (mut header, mut target, mut store) = setup(true);
    header.master_key.pop().unwrap();

    assert_inval_header!("master-key", header.write(&mut target, &mut store));
}

#[test]
fn master_iv_inval_size() {
    let (mut header, mut target, mut store) = setup(true);
    header.master_iv.pop().unwrap();

    assert_inval_header!("master-iv", header.write(&mut target, &mut store));
}

#[test]
fn empty_userdata() {
    let (mut header, mut target, mut store) = setup(true);
    header.userdata.clear();

    assert_eq!(
        header.write(&mut target, &mut store).unwrap(),
        ENCODED_SIZE - 4
    );
}
