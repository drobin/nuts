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

use byteorder::{ByteOrder, NetworkEndian};
use std::io::ErrorKind;

use crate::error::InvalHeaderKind;
use crate::header::ser::HeaderWriter;
use crate::header::Header;
use crate::io::{WriteBasics, WriteExt};
use crate::password::PasswordStore;
use crate::types::{Cipher, DiskType, BLOCK_MIN_SIZE};

fn mk_secret(
    dtype: u8,
    bsize: u32,
    blocks: u64,
    master_key: &[u8],
    master_iv: &[u8],
    hmac_key: &[u8],
    userdata: &[u8],
) -> Vec<u8> {
    let mut secret = vec![0; 512];
    let nbytes = {
        let mut writer = HeaderWriter::new(&mut secret[4..]);

        writer.write_u8(dtype).unwrap();
        writer.write_u32(bsize).unwrap();
        writer.write_u64(blocks).unwrap();
        writer.write_vec(master_key).unwrap();
        writer.write_vec(master_iv).unwrap();
        writer.write_vec(hmac_key).unwrap();
        writer.write_vec(userdata).unwrap();

        writer.offs
    };

    NetworkEndian::write_u32(&mut secret[..4], nbytes as u32);
    secret.resize(4 + nbytes, 0);

    secret
}

struct Data {
    magic: Vec<u8>,
    revision: u8,
    cipher: u8,
    digest: u8,
    wkey_data: Vec<u8>,
    wrapping_iv: Vec<u8>,
    hmac: Vec<u8>,
    secret: Vec<u8>,
}

fn ok_data() -> Data {
    Data {
        magic: vec![b'n', b'u', b't', b's', b'-', b'i', b'o'],
        revision: 1,
        cipher: 0,
        digest: 0xFF,
        wkey_data: vec![0xFF],
        wrapping_iv: vec![0x00, 0x00, 0x00, 0x00],
        hmac: vec![0x00, 0x00, 0x00, 0x00],
        secret: vec![
            0x00, 0x00, 0x00, 0x21, 0, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x12, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 7, 8, 9, 10,
        ],
    }
}

fn mk_data(d: &Data) -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(&d.magic);
    data.push(d.revision);
    data.push(d.cipher);
    data.push(d.digest);
    data.extend_from_slice(&d.wkey_data);
    data.extend_from_slice(&d.wrapping_iv);
    data.extend_from_slice(&d.hmac);
    data.extend_from_slice(&d.secret);

    data
}

#[test]
fn ok() {
    let data = mk_data(&ok_data());
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();

    assert_eq!(nbytes, 56);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.digest, None);
    assert_eq!(header.wrapping_key, None);
    assert_eq!(header.wrapping_iv, []);
    assert_eq!(header.dtype, DiskType::FatZero);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![]);
    assert_eq!(header.master_iv, vec![]);
    assert_eq!(header.hmac_key, vec![]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn ok_ignored_callback() {
    let data = mk_data(&ok_data());
    let mut store = PasswordStore::new();

    store.set_callback(|| panic!("should never be reached"));

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();

    assert_eq!(nbytes, 56);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.digest, None);
    assert_eq!(header.wrapping_key, None);
    assert_eq!(header.wrapping_iv, []);
    assert_eq!(header.dtype, DiskType::FatZero);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![]);
    assert_eq!(header.master_iv, vec![]);
    assert_eq!(header.hmac_key, vec![]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn incomplete() {
    for i in 1..56 {
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

    assert_inval_header!(InvalHeaderKind::InvalMagic, Header::read(&data, &mut store));
}

#[test]
fn bad_revision() {
    let data = mk_data(&Data {
        revision: 0,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalRevision,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bad_cipher() {
    let data = mk_data(&Data {
        cipher: 99,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalCipher,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bad_digest() {
    let data = mk_data(&Data {
        digest: 99,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalDigest,
        Header::read(&data, &mut store)
    );
}

#[test]
fn digest_sha1() {
    let data = mk_data(&Data {
        digest: 1,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalDigest,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bad_wrapping_key_data() {
    let data = mk_data(&Data {
        wkey_data: vec![9],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalWrappingKey,
        Header::read(&data, &mut store)
    );
}

#[test]
fn wrapping_key_data_pbkdf2() {
    let data = mk_data(&Data {
        wkey_data: vec![1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalWrappingKey,
        Header::read(&data, &mut store)
    );
}

#[test]
fn wrapping_iv_not_empty() {
    let data = mk_data(&Data {
        wrapping_iv: vec![0, 0, 0, 1, 9],
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(InvalHeaderKind::InvalIv, Header::read(&data, &mut store));
}

#[test]
fn bad_dtype() {
    let secret = mk_secret(99, BLOCK_MIN_SIZE, 4711, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalDiskType,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bsize_lt_512() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE - 1, 4711, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bsize_inval_modulo() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE + 1, 4711, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        Header::read(&data, &mut store)
    );
}

#[test]
fn bsize_512() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 56);
    assert_eq!(header.bsize, 512);
}

#[test]
fn bsize_1024() {
    let secret = mk_secret(0, 1024, 4711, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 56);
    assert_eq!(header.bsize, 1024);
}

#[test]
fn blocks_0() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 0, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    assert_inval_header!(
        InvalHeaderKind::InvalBlocks,
        Header::read(&data, &mut store)
    );
}

#[test]
fn blocks_1() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 1, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 56);
    assert_eq!(header.blocks, 1);
}

#[test]
fn blocks_2() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 2, &[], &[], &[], &[7, 8, 9, 10]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 56);
    assert_eq!(header.blocks, 2);
}

#[test]
fn master_key_not_empty() {
    let secret = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 1],
        &[],
        &[],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalMasterKey,
        Header::read(&data, &mut store)
    );
}

#[test]
fn master_iv_not_empty() {
    let secret = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[],
        &[b'b'; 1],
        &[],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalMasterIv,
        Header::read(&data, &mut store)
    );
}

#[test]
fn hmac_key_not_empty() {
    let secret = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[],
        &[],
        &[b'c'; 1],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    assert_inval_header!(
        InvalHeaderKind::InvalHmacKey,
        Header::read(&data, &mut store)
    );
}

#[test]
fn empty_userdata() {
    let secret = mk_secret(0, BLOCK_MIN_SIZE, 4711, &[], &[], &[], &[]);
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = PasswordStore::new();

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 52);
    assert_eq!(header.userdata, []);
}
