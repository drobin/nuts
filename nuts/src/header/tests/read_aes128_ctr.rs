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

use ::openssl::pkey::PKey;
use ::openssl::sign::Signer;
use byteorder::{ByteOrder, NetworkEndian};
use std::io::ErrorKind;

use crate::error::{Error, InvalHeaderKind};
use crate::header::ser::HeaderWriter;
use crate::header::Header;
use crate::io::{WriteBasics, WriteExt};
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType, WrappingKey, BLOCK_MIN_SIZE};

fn callback() -> Result<Vec<u8>> {
    Ok(vec![b'1', b'2', b'3'])
}

fn mk_secret(
    dtype: u8,
    bsize: u32,
    blocks: u64,
    master_key: &[u8],
    master_iv: &[u8],
    hmac_key: &[u8],
    userdata: &[u8],
) -> (Vec<u8>, Vec<u8>) {
    // the plain secret
    let mut plain_secret = vec![0; 512];
    let nbytes = {
        let mut writer = HeaderWriter::new(&mut plain_secret);

        writer.write_u8(dtype).unwrap();
        writer.write_u32(bsize).unwrap();
        writer.write_u64(blocks).unwrap();
        writer.write_vec(master_key).unwrap();
        writer.write_vec(master_iv).unwrap();
        writer.write_vec(hmac_key).unwrap();
        writer.write_vec(userdata).unwrap();

        writer.offs
    };
    plain_secret.resize(nbytes, 0);

    // encrypt into secret
    let wkey = WrappingKey::Pbkdf2 {
        iterations: 4711,
        salt: vec![1, 2, 3],
    }
    .create_wrapping_key(b"123", Digest::Sha1)
    .unwrap();

    let wiv = [
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    ];

    let mut secret = vec![0; 4 + plain_secret.len()];
    NetworkEndian::write_u32(&mut secret[..4], plain_secret.len() as u32);
    Cipher::Aes128Ctr
        .encrypt(&plain_secret, &mut secret[4..], &wkey, &wiv)
        .unwrap();

    // create hmac
    let pkey = PKey::hmac(&[b'c'; 20]).unwrap();
    let mut signer = Signer::new(Digest::Sha1.to_openssl(), &pkey).unwrap();
    let mut hmac = vec![0; 4 + Digest::Sha1.size() as usize];

    NetworkEndian::write_u32(&mut hmac[..4], Digest::Sha1.size());
    signer.sign_oneshot(&mut hmac[4..], &plain_secret).unwrap();

    (secret, hmac)
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
        cipher: 1,
        digest: 1,
        wkey_data: vec![1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3],
        wrapping_iv: vec![
            0x00, 0x00, 0x00, 0x10, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
        ],
        hmac: vec![
            0, 0, 0, 20, 208, 98, 13, 189, 168, 244, 235, 189, 4, 29, 124, 252, 76, 223, 98, 90,
            100, 202, 104, 26,
        ],
        secret: vec![
            0, 0, 0, 85, 21, 41, 218, 239, 228, 245, 41, 150, 29, 113, 119, 117, 150, 178, 29, 147,
            144, 100, 134, 111, 47, 5, 92, 46, 136, 34, 229, 149, 229, 214, 30, 226, 197, 251, 52,
            53, 192, 49, 150, 111, 85, 161, 122, 173, 223, 205, 185, 225, 78, 217, 224, 146, 31,
            186, 146, 196, 199, 222, 232, 79, 170, 98, 176, 179, 202, 46, 0, 142, 172, 167, 183,
            51, 21, 62, 115, 101, 214, 190, 72, 53, 163, 199, 77, 238, 42,
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
    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();

    assert_eq!(nbytes, 155);
    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::Aes128Ctr);
    assert_eq!(header.digest, Some(Digest::Sha1));
    assert_eq!(
        header.wrapping_key,
        Some(WrappingKey::Pbkdf2 {
            iterations: 4711,
            salt: vec![1, 2, 3]
        })
    );
    assert_eq!(
        header.wrapping_iv,
        [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]
    );
    assert_eq!(header.dtype, DiskType::FatRandom);
    assert_eq!(header.bsize, 512);
    assert_eq!(header.blocks, 4711);
    assert_eq!(header.master_key, vec![b'a'; 16]);
    assert_eq!(header.master_iv, vec![b'b'; 16]);
    assert_eq!(header.hmac_key, vec![b'c'; 20]);
    assert_eq!(header.userdata, [7, 8, 9, 10]);
}

#[test]
fn missing_callback() {
    let data = mk_data(&ok_data());
    let none: Option<fn() -> Result<Vec<u8>>> = None;

    assert_error!(Error::NoPassword, Header::read(&data, none));
}

#[test]
fn incomplete() {
    for i in 1..155 {
        let data = &mk_data(&ok_data())[..i];
        assert_io_error!(
            ErrorKind::UnexpectedEof,
            Header::read(&data, Some(callback))
        );
    }
}

#[test]
fn bad_magic() {
    let data = mk_data(&Data {
        magic: vec![b'X', b'u', b't', b's', b'-', b'i', b'o'],
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalMagic,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_revision() {
    let data = mk_data(&Data {
        revision: 0,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalRevision,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_cipher() {
    let data = mk_data(&Data {
        cipher: 99,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalCipher,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_digest() {
    let data = mk_data(&Data {
        digest: 99,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalDigest,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn digest_none() {
    let data = mk_data(&Data {
        digest: 0xFF,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalDigest,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_wrapping_key_data() {
    let data = mk_data(&Data {
        wkey_data: vec![9],
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalWrappingKey,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn wrapping_key_data_none() {
    let data = mk_data(&Data {
        wkey_data: vec![0xFF],
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalWrappingKey,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn wrapping_iv_inval_size() {
    let data = mk_data(&Data {
        wrapping_iv: vec![0, 0, 0, 15, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalIv,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_hmac() {
    let data = mk_data(&Data {
        hmac: vec![
            0, 0, 0, 19, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        ],
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalHmac,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bad_hmac_mismatch() {
    let data = mk_data(&Data {
        hmac: vec![
            0, 0, 0, 20, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        ],
        ..ok_data()
    });

    assert_error!(Error::HmacMismatch, Header::read(&data, Some(callback)));
}

#[test]
fn bad_dtype() {
    let (secret, hmac) = mk_secret(
        99,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalDiskType,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bsize_lt_512() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE - 1,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });
    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bsize_inval_modulo() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE + 1,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });
    assert_inval_header!(
        InvalHeaderKind::InvalBlockSize,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn bsize_512() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();
    assert_eq!(nbytes, 155);
    assert_eq!(header.bsize, 512);
}

#[test]
fn bsize_1024() {
    let (secret, hmac) = mk_secret(
        0,
        1024,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();
    assert_eq!(nbytes, 155);
    assert_eq!(header.bsize, 1024);
}

#[test]
fn blocks_0() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        0,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });
    assert_inval_header!(
        InvalHeaderKind::InvalBlocks,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn blocks_1() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        1,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });
    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();
    assert_eq!(nbytes, 155);
    assert_eq!(header.blocks, 1);
}

#[test]
fn blocks_2() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        2,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });
    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();
    assert_eq!(nbytes, 155);
    assert_eq!(header.blocks, 2);
}

#[test]
fn master_key_inval_size() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 15],
        &[b'b'; 16],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalMasterKey,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn master_iv_inval_size() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 15],
        &[b'c'; 20],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalMasterIv,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn hmac_key_inval_size() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 19],
        &[7, 8, 9, 10],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    assert_inval_header!(
        InvalHeaderKind::InvalHmacKey,
        Header::read(&data, Some(callback))
    );
}

#[test]
fn empty_userdata() {
    let (secret, hmac) = mk_secret(
        0,
        BLOCK_MIN_SIZE,
        4711,
        &[b'a'; 16],
        &[b'b'; 16],
        &[b'c'; 20],
        &[],
    );
    let data = mk_data(&Data {
        hmac,
        secret,
        ..ok_data()
    });

    let (header, nbytes) = Header::read(&data, Some(callback)).unwrap();
    assert_eq!(nbytes, 155 - 4);
    assert_eq!(header.userdata, []);
}
