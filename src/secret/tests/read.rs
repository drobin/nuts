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

use crate::secret::Secret;
use crate::types::DiskType;

struct Data {
    dtype: u8,
    bsize: u32,
    blocks: u64,
    master_key: [u8; 8],
    master_key_size: usize,
    master_iv: [u8; 8],
    master_iv_size: usize,
    hmac_key: [u8; 8],
    hmac_key_size: usize,
    userdata: [u8; 8],
    userdata_size: usize,
}

const OK_DATA: Data = Data {
    dtype: 0,
    bsize: 512,
    blocks: 4711,
    master_key: [0, 0, 0, 1, 1, 0, 0, 0],
    master_key_size: 5,
    master_iv: [0, 0, 0, 2, 2, 3, 0, 0],
    master_iv_size: 6,
    hmac_key: [0, 0, 0, 3, 4, 5, 6, 0],
    hmac_key_size: 7,
    userdata: [0, 0, 0, 4, 7, 8, 9, 10],
    userdata_size: 8,
};

fn mk_data(d: &Data) -> Vec<u8> {
    let mut data = Vec::new();
    let mut buf = [0; 16];

    data.push(d.dtype);

    NetworkEndian::write_u32(&mut buf, d.bsize);
    data.extend_from_slice(&buf[..4]);

    NetworkEndian::write_u64(&mut buf, d.blocks);
    data.extend_from_slice(&buf[..8]);

    data.extend_from_slice(&d.master_key[..d.master_key_size]);
    data.extend_from_slice(&d.master_iv[..d.master_iv_size]);
    data.extend_from_slice(&d.hmac_key[..d.hmac_key_size]);
    data.extend_from_slice(&d.userdata[..d.userdata_size]);

    data
}

fn mk_data_size(d: &Data, size: usize) -> Vec<u8> {
    let mut data = mk_data(d);

    data.resize(size, 0xFF);

    data
}

#[test]
fn ok() {
    let data = mk_data(&OK_DATA);
    let (secret, offset) = Secret::read(&data).unwrap();

    assert_eq!(offset, 39);
    assert_eq!(secret.dtype, DiskType::FatZero);
    assert_eq!(secret.bsize, 512);
    assert_eq!(secret.blocks, 4711);
    assert_eq!(secret.master_key, [1]);
    assert_eq!(secret.master_iv, [2, 3]);
    assert_eq!(secret.hmac_key, [4, 5, 6]);
    assert_eq!(secret.userdata, [7, 8, 9, 10]);
}

#[test]
fn incomplete() {
    for i in 1..39 {
        let data = mk_data_size(&OK_DATA, i);
        let err = format!("{:?}", Secret::read(&data).unwrap_err());
        assert_eq!(err, "NoData");
    }
}

#[test]
fn dtype_fat_zero() {
    let data = mk_data(&Data {
        dtype: 0,
        ..OK_DATA
    });

    let (secret, _) = Secret::read(&data).unwrap();
    assert_eq!(secret.dtype, DiskType::FatZero);
}

#[test]
fn dtype_fat_random() {
    let data = mk_data(&Data {
        dtype: 1,
        ..OK_DATA
    });

    let (secret, _) = Secret::read(&data).unwrap();
    assert_eq!(secret.dtype, DiskType::FatRandom);
}

#[test]
fn dtype_thin_zero() {
    let data = mk_data(&Data {
        dtype: 2,
        ..OK_DATA
    });

    let (secret, _) = Secret::read(&data).unwrap();
    assert_eq!(secret.dtype, DiskType::ThinZero);
}

#[test]
fn dtype_thin_random() {
    let data = mk_data(&Data {
        dtype: 3,
        ..OK_DATA
    });

    let (secret, _) = Secret::read(&data).unwrap();
    assert_eq!(secret.dtype, DiskType::ThinRandom);
}

#[test]
fn bad_dtype() {
    let data = mk_data(&Data {
        dtype: 4,
        ..OK_DATA
    });

    let err = format!("{:?}", Secret::read(&data).unwrap_err());
    assert_eq!(err, "InvalHeader(InvalDiskType)");
}
