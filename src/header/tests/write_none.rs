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

use crate::header::Header;
use crate::password::PasswordStore;
use crate::types::{Cipher, Digest, DiskType, WrappingKey, BLOCK_MIN_SIZE};

const ENCODED_SIZE: u32 = 48;
const ENCODED_SECRET: [u8; 33] = [
    0, 0, 0, 29, // secret-size
    1,  // dtype
    0, 0, 2, 0, // bsize
    0, 0, 0, 0, 0, 0, 18, 103, // blocks
    0, 0, 0, 0, // master-key (incl. len)
    0, 0, 0, 0, // master-iv (incl. len)
    0, 0, 0, 4, 7, 8, 9, 10, // userdata (incl. len)
];

fn ok_header() -> Header {
    Header {
        revision: 1,
        cipher: Cipher::None,
        digest: None,
        wrapping_key: None,
        wrapping_iv: vec![],
        dtype: DiskType::FatRandom,
        bsize: BLOCK_MIN_SIZE,
        blocks: 4711,
        master_key: secure_vec![],
        master_iv: secure_vec![],
        userdata: vec![7, 8, 9, 10],
    }
}

fn setup() -> (Header, [u8; 256], PasswordStore) {
    (ok_header(), [0; 256], PasswordStore::new())
}

#[test]
fn ok() {
    let (header, mut target, mut store) = setup();

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 0); // cipher
    assert_eq!(target[9], 0xFF); // digest
    assert_eq!(target[10], 0xFF); // pbkdf2
    assert_eq!(target[11..15], [0x00, 0x00, 0x00, 0x00]); // wrapping_iv
    assert_eq!(target[15..47], ENCODED_SECRET[..32]); // secret, part I
    assert_eq!(&target[47..48], &ENCODED_SECRET[32..]); // secret, part II
}

#[test]
fn ok_ignored_callback() {
    let (header, mut target, mut store) = setup();

    store.set_callback(|| panic!("should never be reached"));

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 0); // cipher
    assert_eq!(target[9], 0xFF); // digest
    assert_eq!(target[10], 0xFF); // pbkdf2
    assert_eq!(target[11..15], [0x00, 0x00, 0x00, 0x00]); // wrapping_iv
    assert_eq!(target[15..47], ENCODED_SECRET[..32]); // secret, part I
    assert_eq!(&target[47..48], &ENCODED_SECRET[32..]); // secret, part II
}

#[test]
fn no_space() {
    let (header, mut target, mut store) = setup();

    for i in 1..ENCODED_SIZE as usize {
        assert_io_error!(
            ErrorKind::WriteZero,
            header.write(&mut target.get_mut(..i).unwrap(), &mut store)
        );
    }
}

#[test]
fn digest_none() {
    let (mut header, mut target, mut store) = setup();
    header.digest = None;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[9], 0xFF);
}

#[test]
fn digest_sha1() {
    let (mut header, mut target, mut store) = setup();
    header.digest = Some(Digest::Sha1);

    assert_inval_header!("digest", header.write(&mut target, &mut store));
}

#[test]
fn wrapping_key_data_none() {
    let (mut header, mut target, mut store) = setup();
    header.wrapping_key = None;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[10], 0xFF);
}

#[test]
fn wrapping_key_data_pbkdf2() {
    let (mut header, mut target, mut store) = setup();
    header.wrapping_key = Some(WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 4711,
        salt: vec![1, 2, 3],
    });

    assert_inval_header!("wrapping-key", header.write(&mut target, &mut store));
}

#[test]
fn wrapping_iv_not_empty() {
    let (mut header, mut target, mut store) = setup();
    header.wrapping_iv.insert(0, b'x');

    assert_inval_header!("iv", header.write(&mut target, &mut store));
}

#[test]
fn wrapping_iv_empty() {
    let (header, mut target, mut store) = setup();

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
    assert_eq!(target[11..15], [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn bsize_lt_512() {
    let (mut header, mut target, mut store) = setup();
    header.bsize = BLOCK_MIN_SIZE - 1;

    assert_inval_header!("block-size", header.write(&mut target, &mut store));
}

#[test]
fn bsize_inval_modulo() {
    let (mut header, mut target, mut store) = setup();
    header.bsize = BLOCK_MIN_SIZE + 1;

    assert_inval_header!("block-size", header.write(&mut target, &mut store));
}

#[test]
fn bsize_512() {
    let (mut header, mut target, mut store) = setup();
    header.bsize = 512;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn bsize_1024() {
    let (mut header, mut target, mut store) = setup();
    header.bsize = 1024;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_0() {
    let (mut header, mut target, mut store) = setup();
    header.blocks = 0;

    assert_inval_header!("blocks", header.write(&mut target, &mut store));
}

#[test]
fn blocks_1() {
    let (mut header, mut target, mut store) = setup();
    header.blocks = 1;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn blocks_2() {
    let (mut header, mut target, mut store) = setup();
    header.blocks = 2;

    assert_eq!(header.write(&mut target, &mut store).unwrap(), ENCODED_SIZE);
}

#[test]
fn master_key_not_empty() {
    let (mut header, mut target, mut store) = setup();
    header.master_key.insert(0, b'x');

    assert_inval_header!("master-key", header.write(&mut target, &mut store));
}

#[test]
fn master_iv_not_empty() {
    let (mut header, mut target, mut store) = setup();
    header.master_iv.insert(0, b'x');

    assert_inval_header!("master-iv", header.write(&mut target, &mut store));
}

#[test]
fn empty_userdata() {
    let (mut header, mut target, mut store) = setup();
    header.userdata.clear();

    assert_eq!(
        header.write(&mut target, &mut store).unwrap(),
        ENCODED_SIZE - 4
    );
}
