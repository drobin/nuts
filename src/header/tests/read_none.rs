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

use std::io::{Cursor, ErrorKind};

use crate::header::Header;
use crate::io::BinaryWrite;
use crate::password::PasswordStore;
use crate::types::{Cipher, DiskType, BLOCK_MIN_SIZE};

fn mk_secret(
    dtype: u8,
    bsize: u32,
    blocks: u64,
    master_key: &[u8],
    master_iv: &[u8],
    userdata: &[u8],
) -> Vec<u8> {
    let mut secret = vec![0; 512];
    let nbytes = {
        let mut cursor = Cursor::new(&mut secret[4..]);

        cursor.write_binary(&dtype).unwrap();
        cursor.write_binary(&bsize).unwrap();
        cursor.write_binary(&blocks).unwrap();
        cursor.write_binary(&master_key.to_vec()).unwrap();
        cursor.write_binary(&master_iv.to_vec()).unwrap();
        cursor.write_binary(&userdata.to_vec()).unwrap();

        cursor.position() as usize
    };

    let len = &((nbytes as u32).to_be_bytes()[..]);
    secret[..4].copy_from_slice(len);
    secret.resize(4 + nbytes, 0);

    secret
}

struct Data {
    magic: Vec<u8>,
    revision: u8,
    cipher: u8,
    wkey_data: Vec<u8>,
    wrapping_iv: Vec<u8>,
    secret: Vec<u8>,
}

fn ok_data() -> Data {
    Data {
        magic: vec![b'n', b'u', b't', b's', b'-', b'i', b'o'],
        revision: 1,
        cipher: 0,
        wkey_data: vec![0xFF],
        wrapping_iv: vec![0x00, 0x00, 0x00, 0x00],
        secret: vec![
            0x00, 0x00, 0x00, 0x1D, // length
            0,    // dtype
            0x00, 0x00, 0x02, 0x00, // bsize
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x67, // blocks
            0x00, 0x00, 0x00, 0x00, // master-key
            0x00, 0x00, 0x00, 0x00, // master-iv
            0x00, 0x00, 0x00, 0x04, 7, 8, 9, 10, // userdata
        ],
    }
}

fn mk_data(d: &Data) -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&d.magic);
    data.push(d.revision);
    data.push(d.cipher);
    data.extend_from_slice(&d.wkey_data);
    data.extend_from_slice(&d.wrapping_iv);
    data.extend_from_slice(&d.secret);

    data
}

#[test]
fn ok() {
    let data = mk_data(&ok_data());
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();

    assert_eq!(nbytes, 47);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.wrapping_key, None);
    assert_eq!(header.wrapping_iv, []);
    assert_eq!(header.dtype, DiskType::FatZero);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![]);
    assert_eq!(header.master_iv, vec![]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn ok_ignored_callback() {
    let data = mk_data(&ok_data());
    let mut store = PasswordStore::new();

    store.set_callback(|| panic!("should never be reached"));

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();

    assert_eq!(nbytes, 47);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.wrapping_key, None);
    assert_eq!(header.wrapping_iv, []);
    assert_eq!(header.dtype, DiskType::FatZero);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![]);
    assert_eq!(header.master_iv, vec![]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn incomplete() {
    for i in 1..47 {
        let data = &mk_data(&ok_data())[..i];
        let mut store = PasswordStore::new();
        assert_io_error!(ErrorKind::UnexpectedEof, Header::read(&data, &mut store));
    }
}

#[test]
fn bad_magic() {
    let data = mk_data(&Data {
        magic: vec![b'X', b'u', b't', b's', b'-', b'i', b'o'],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("magic", Header::read(&data, &mut store));
}

#[test]
fn bad_revision() {
    let data = mk_data(&Data {
        revision: 0,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("revision", Header::read(&data, &mut store));
}

#[test]
fn bad_cipher() {
    let data = mk_data(&Data {
        cipher: 99,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("cipher", Header::read(&data, &mut store));
}

#[test]
fn bad_wrapping_key_data() {
    let data = mk_data(&Data {
        wkey_data: vec![9],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("wrapping-key", Header::read(&data, &mut store));
}

#[test]
fn wrapping_key_data_pbkdf2() {
    let data = mk_data(&Data {
        wkey_data: vec![
            1, 0x01, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
        ],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("wrapping-key", Header::read(&data, &mut store));
}

#[test]
fn wrapping_iv_not_empty() {
    let data = mk_data(&Data {
        wrapping_iv: vec![0, 0, 0, 1, 9],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("iv", Header::read(&data, &mut store));
}

#[test]
fn bad_dtype() {
    let secret = mk_secret(99, BLOCK_MIN_SIZE, 4711, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("disk-type", Header::read(&data, &mut store));
}

#[test]
fn bsize_lt_512() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE - 1, 4711, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!("block-size", Header::read(&data, &mut store));
}

#[test]
fn bsize_inval_modulo() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE + 1, 4711, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!("block-size", Header::read(&data, &mut store));
}

#[test]
fn bsize_512() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 47);
    assert_eq!(header.bsize, 512);
}

#[test]
fn bsize_1024() {
    let secret = mk_secret(0, 1024, 4711, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 47);
    assert_eq!(header.bsize, 1024);
}

#[test]
fn blocks_0() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 0, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!("blocks", Header::read(&data, &mut store));
}

#[test]
fn blocks_1() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 1, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 47);
    assert_eq!(header.blocks, 1);
}

#[test]
fn blocks_2() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 2, &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 47);
    assert_eq!(header.blocks, 2);
}

#[test]
fn master_key_not_empty() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[b'a'; 1], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("master-key", Header::read(&data, &mut store));
}

#[test]
fn master_iv_not_empty() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[], &[b'b'; 1], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!("master-iv", Header::read(&data, &mut store));
}

#[test]
fn empty_userdata() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[], &[], &[]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 43);
    assert_eq!(header.userdata, []);
}
