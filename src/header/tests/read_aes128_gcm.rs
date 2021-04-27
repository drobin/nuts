// MIT License
//
// Copyright (c) 2021 Robin Doer
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

use std::io::{Cursor, ErrorKind, Write};

use crate::error::Error;
use crate::header::Header;
use crate::io::BinaryWrite;
use crate::password::PasswordStore;
use crate::types::{Cipher, Digest, DiskType, WrappingKey, BLOCK_MIN_SIZE};

fn mk_secret(
    magic: &[u8],
    dtype: u8,
    bsize: u32,
    blocks: u64,
    master_key: &[u8],
    master_iv: &[u8],
    userdata: &[u8],
) -> Vec<u8> {
    // the plain secret
    let mut plain_secret = vec![0; 512];
    let nbytes = {
        let mut cursor = Cursor::new(&mut plain_secret);

        cursor.write_all(magic).unwrap();
        cursor.write_binary(&dtype).unwrap();
        cursor.write_binary(&bsize).unwrap();
        cursor.write_binary(&blocks).unwrap();
        cursor.write_binary(&master_key.to_vec()).unwrap();
        cursor.write_binary(&master_iv.to_vec()).unwrap();
        cursor.write_binary(&userdata.to_vec()).unwrap();

        cursor.position() as usize
    };
    plain_secret.resize(nbytes, 0);

    // encrypt into secret
    let wkey = WrappingKey::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 4711,
        salt: vec![1, 2, 3],
    }
    .create_wrapping_key(b"123")
    .unwrap();

    let wiv = [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];

    let cipher_secret = Cipher::Aes128Gcm
        .encrypt(&plain_secret, &wkey, &wiv)
        .unwrap();

    let mut secret = (cipher_secret.len() as u32).to_be_bytes().to_vec();
    secret.extend(cipher_secret.iter());

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
        cipher: 2,
        wkey_data: vec![
            1, 0x01, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3,
        ],
        wrapping_iv: vec![
            0x00, 0x00, 0x00, 0x0C, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        ],
        secret: vec![
            0, 0, 0, 84, 154, 41, 211, 179, 118, 226, 161, 149, 137, 43, 50, 183, 157, 123, 241,
            236, 2, 66, 199, 128, 86, 76, 85, 222, 114, 226, 29, 55, 103, 176, 59, 204, 136, 27,
            56, 131, 100, 0, 99, 41, 160, 168, 235, 176, 57, 66, 191, 59, 190, 41, 106, 54, 95,
            100, 197, 219, 62, 23, 27, 140, 82, 73, 87, 249, 175, 211, 249, 11, 140, 26, 210, 168,
            222, 159, 97, 115, 72, 206, 182, 128, 146, 136, 110, 210,
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

fn setup_store(password: bool) -> PasswordStore {
    let mut store = PasswordStore::new();

    if password {
        store.set_value(secure_vec![b'1', b'2', b'3']);
    }

    store
}

#[test]
fn ok() {
    let data = mk_data(&ok_data());
    let mut store = setup_store(true);
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();

    assert_eq!(nbytes, 126);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::Aes128Gcm);
    assert_eq!(
        header.wrapping_key,
        Some(WrappingKey::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 4711,
            salt: vec![1, 2, 3]
        })
    );
    assert_eq!(
        header.wrapping_iv,
        [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
    );
    assert_eq!(header.dtype, DiskType::FatRandom);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![b'a'; 16]);
    assert_eq!(header.master_iv, vec![b'b'; 12]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn missing_callback() {
    let data = mk_data(&ok_data());
    let mut store = setup_store(false);

    assert_error!(Error::NoPassword, Header::read(&data, &mut store));
}

#[test]
fn incomplete() {
    for i in 1..126 {
        let data = &mk_data(&ok_data())[..i];
        let mut store = setup_store(true);
        assert_io_error!(ErrorKind::UnexpectedEof, Header::read(&data, &mut store));
    }
}

#[test]
fn bad_header_magic() {
    let data = mk_data(&Data {
        magic: vec![b'X', b'u', b't', b's', b'-', b'i', b'o'],
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("magic", Header::read(&data, &mut store));
}

#[test]
fn bad_secret_magic() {
    let secret = mk_secret(
        b"nuts-secreX",
        1,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    let err = Header::read(&data, &mut store).unwrap_err();
    assert_eq!(format!("{:?}", err), "WrongPassword");
}

#[test]
fn bad_revision() {
    let data = mk_data(&Data {
        revision: 0,
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("revision", Header::read(&data, &mut store));
}

#[test]
fn bad_cipher() {
    let data = mk_data(&Data {
        cipher: 99,
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("cipher", Header::read(&data, &mut store));
}

#[test]
fn bad_wrapping_key_data() {
    let data = mk_data(&Data {
        wkey_data: vec![9],
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("wrapping-key", Header::read(&data, &mut store));
}

#[test]
fn wrapping_key_data_none() {
    let data = mk_data(&Data {
        wkey_data: vec![0xFF],
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("wrapping-key", Header::read(&data, &mut store));
}

#[test]
fn wrapping_iv_inval_size() {
    let data = mk_data(&Data {
        wrapping_iv: vec![0, 0, 0, 15, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("iv", Header::read(&data, &mut store));
}

#[test]
fn bad_dtype() {
    let secret = mk_secret(
        b"nuts-secret",
        99,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("disk-type", Header::read(&data, &mut store));
}

#[test]
fn bsize_lt_512() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE - 1,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);
    assert_inval_header!("block-size", Header::read(&data, &mut store));
}

#[test]
fn bsize_inval_modulo() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE + 1,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);
    assert_inval_header!("block-size", Header::read(&data, &mut store));
}

#[test]
fn bsize_512() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 126);
    assert_eq!(header.bsize, 512);
}

#[test]
fn bsize_1024() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        1024,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 126);
    assert_eq!(header.bsize, 1024);
}

#[test]
fn blocks_0() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        0,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);
    assert_inval_header!("blocks", Header::read(&data, &mut store));
}

#[test]
fn blocks_1() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        1,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 126);
    assert_eq!(header.blocks, 1);
}

#[test]
fn blocks_2() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        2,
        &[b'a'; 16],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);
    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 126);
    assert_eq!(header.blocks, 2);
}

#[test]
fn master_key_inval_size() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 15],
        &[b'b'; 12],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("master-key", Header::read(&data, &mut store));
}

#[test]
fn master_iv_inval_size() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 11],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    assert_inval_header!("master-iv", Header::read(&data, &mut store));
}

#[test]
fn empty_userdata() {
    let secret = mk_secret(
        b"nuts-secret",
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 12],
        &[],
    );
    let data = mk_data(&Data {
        secret,
        ..ok_data()
    });
    let mut store = setup_store(true);

    let (header, nbytes) = Header::read(&data, &mut store).unwrap();
    assert_eq!(nbytes, 126 - 4);
    assert_eq!(header.userdata, []);
}
