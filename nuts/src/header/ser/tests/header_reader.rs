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

use std::io::{ErrorKind, Read};

use crate::error::Error;
use crate::header::ser::HeaderReader;
use crate::types::{Cipher, Digest, DiskType, WrappingKeyData};

fn read_remaining(reader: &mut HeaderReader) -> Vec<u8> {
    let mut v = vec![];
    reader.read_to_end(&mut v).unwrap();
    v
}

#[test]
fn revision_no_data() {
    let data = [0; 0];
    let mut reader = HeaderReader::new(&data);

    if let Error::IoError(err) = reader.read_revision().unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn revision_inval() {
    let data = [2];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_revision().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalRevision)");
}

#[test]
fn revision_ok_complete() {
    let data = [1];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_revision().unwrap(), 1);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn revision_ok_remaining() {
    let data = [1, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_revision().unwrap(), 1);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn magic_no_data() {
    let data = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

    for i in 0..7 {
        let slice: &[u8] = data.get(..i).unwrap();
        let mut reader = HeaderReader::new(&slice);

        if let Error::IoError(err) = reader.read_magic().unwrap_err() {
            assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
        } else {
            panic!("invalid error");
        }
    }
}

#[test]
fn magic_inval() {
    let data = [b'x', b'u', b't', b's', b'-', b'i', b'o'];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_magic().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalMagic)");
}

#[test]
fn magic_ok_complete() {
    let data = [b'n', b'u', b't', b's', b'-', b'i', b'o'];
    let mut reader = HeaderReader::new(&data);

    reader.read_magic().unwrap();
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn magic_ok_remaining() {
    let data = [b'n', b'u', b't', b's', b'-', b'i', b'o', b'x'];
    let mut reader = HeaderReader::new(&data);

    reader.read_magic().unwrap();
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn cipher_no_data() {
    let data = [0; 0];
    let mut reader = HeaderReader::new(&data);

    if let Error::IoError(err) = reader.read_cipher().unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn cipher_inval() {
    let data = [2];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_cipher().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalCipher)");
}

#[test]
fn cipher_none_complete() {
    let data = [0];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_cipher().unwrap(), Cipher::None);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn cipher_none_remaining() {
    let data = [0, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_cipher().unwrap(), Cipher::None);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn cipher_aes128_ctr_complete() {
    let data = [1];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_cipher().unwrap(), Cipher::Aes128Ctr);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn cipher_aes128_ctr_remaining() {
    let data = [1, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_cipher().unwrap(), Cipher::Aes128Ctr);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn digest_no_data() {
    let data = [0; 0];
    let mut reader = HeaderReader::new(&data);

    if let Error::IoError(err) = reader.read_digest().unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn digest_inval() {
    let data = [2];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_digest().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDigest)");
}

#[test]
fn digest_none_complete() {
    let data = [0xff];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_digest().unwrap(), None);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn digest_none_remaining() {
    let data = [0xff, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_digest().unwrap(), None);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn digest_sha1_complete() {
    let data = [1];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_digest().unwrap(), Some(Digest::Sha1));
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn digest_sha1_remaining() {
    let data = [1, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_digest().unwrap(), Some(Digest::Sha1));
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn dtype_no_data() {
    let data = [0; 0];
    let mut reader = HeaderReader::new(&data);

    if let Error::IoError(err) = reader.read_dtype().unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn dtype_inval() {
    let data = [4];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_dtype().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDiskType)");
}

#[test]
fn dtype_fat_zero_complete() {
    let data = [0];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::FatZero);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn dtype_fat_zero_remaining() {
    let data = [0, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::FatZero);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn dtype_fat_random_complete() {
    let data = [1];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::FatRandom);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn dtype_fat_random_remaining() {
    let data = [1, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::FatRandom);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn dtype_thin_zero_complete() {
    let data = [2];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::ThinZero);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn dtype_thin_zero_remaining() {
    let data = [2, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::ThinZero);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn dtype_thin_random_complete() {
    let data = [3];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::ThinRandom);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn dtype_thin_random_remaining() {
    let data = [3, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_dtype().unwrap(), DiskType::ThinRandom);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn wrapping_key_inval() {
    let data = [2];
    let mut reader = HeaderReader::new(&data);

    let err = reader.read_wrapping_key().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalWrappingKey)");
}

#[test]
fn wrapping_key_pbkdf2_no_data() {
    let data = [1, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3];

    for i in 0..12 {
        let slice = data.get(..i).unwrap();
        let mut reader = HeaderReader::new(slice);

        if let Error::IoError(err) = reader.read_wrapping_key().unwrap_err() {
            assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
        } else {
            panic!("invalid error");
        }
    }
}

#[test]
fn wrapping_key_pbkdf2_complete() {
    let data = [1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3];
    let mut reader = HeaderReader::new(&data);

    let wkey = reader.read_wrapping_key().unwrap();
    assert_eq!(
        wkey,
        Some(WrappingKeyData::Pbkdf2 {
            iterations: 65536,
            salt: vec![1, 2, 3]
        })
    );
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn wrapping_key_pbkdf2_remaining() {
    let data = [
        1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, b'x',
    ];
    let mut reader = HeaderReader::new(&data);

    let wkey = reader.read_wrapping_key().unwrap();
    assert_eq!(
        wkey,
        Some(WrappingKeyData::Pbkdf2 {
            iterations: 65536,
            salt: vec![1, 2, 3]
        })
    );
    assert_eq!(read_remaining(&mut reader), [b'x']);
}

#[test]
fn wrapping_key_none_complete() {
    let data = [0xff];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_wrapping_key().unwrap(), None);
    assert_eq!(read_remaining(&mut reader), []);
}

#[test]
fn wrapping_key_none_remaining() {
    let data = [0xff, b'x'];
    let mut reader = HeaderReader::new(&data);

    assert_eq!(reader.read_wrapping_key().unwrap(), None);
    assert_eq!(read_remaining(&mut reader), [b'x']);
}
