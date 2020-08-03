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

use crate::header::ser::ReadHeader;
use crate::types::{Cipher, Digest, DiskType};
use crate::wkey::{Pbkdf2Data, WrappingKeyData};

#[test]
fn u8_no_data() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let err = slice.read_u8().unwrap_err();
    assert_eq!(format!("{:?}", err), "NoData");
}

#[test]
fn u8_complete() {
    let data = [6];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u8().unwrap(), 6);
    assert_eq!(slice.len(), 0);
}

#[test]
fn u8_remaining() {
    let data = [1, 2, 3];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u8().unwrap(), 1);
    assert_eq!(slice, [2, 3]);
}

#[test]
fn u32_no_data() {
    let mut data = [0x12, 0x67, 0x13, 0x68];

    for i in 0..4 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_u32().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        assert_eq!(slice.len(), i);
    }
}

#[test]
fn u32_complete() {
    let data = [0x12, 0x67, 0x13, 0x68];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u32().unwrap(), 308_745_064);
    assert_eq!(slice.len(), 0);
}

#[test]
fn u32_remaining() {
    let data = [0x12, 0x67, 0x13, 0x68, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u32().unwrap(), 308_745_064);
    assert_eq!(slice, [b'x']);
}

#[test]
fn u64_no_data() {
    let mut data = [0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70];

    for i in 0..8 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_u64().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        assert_eq!(slice.len(), i);
    }
}

#[test]
fn u64_complete() {
    let data = [0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u64().unwrap(), 1_326_049_953_023_858_032);
    assert_eq!(slice.len(), 0);
}

#[test]
fn u64_remaining() {
    let data = [0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_u64().unwrap(), 1_326_049_953_023_858_032);
    assert_eq!(slice, [b'x']);
}

#[test]
fn array_empty_complete() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let arr = slice.read_array(0).unwrap();
    assert_eq!(arr.len(), 0);
}

#[test]
fn array_empty_remaining() {
    let data = [b'x'];
    let mut slice: &[u8] = &data;

    let arr = slice.read_array(0).unwrap();
    assert_eq!(arr.len(), 0);
    assert_eq!(slice, [b'x'])
}

#[test]
fn array_non_empty_no_data() {
    let mut data = [1, 2, 3];

    for i in 0..2 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_array(3).unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        assert_eq!(slice.len(), 0);
    }
}

#[test]
fn array_non_empty_complete() {
    let data = [1, 2, 3];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_array(3).unwrap(), [1, 2, 3]);
    assert_eq!(slice.len(), 0);
}

#[test]
fn array_non_empty_remaining() {
    let data = [1, 2, 3, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_array(3).unwrap(), [1, 2, 3]);
    assert_eq!(slice, [b'x']);
}

#[test]
fn vec_empty_no_data() {
    let mut data = [0, 0, 0, 0];

    for i in 0..4 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_vec().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        assert_eq!(slice.len(), i);
    }
}

#[test]
fn vec_empty_complete() {
    let data = [0, 0, 0, 0];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_vec().unwrap(), []);
    assert_eq!(slice.len(), 0);
}

#[test]
fn vec_empty_remaining() {
    let data = [0, 0, 0, 0, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_vec().unwrap(), []);
    assert_eq!(slice, [b'x']);
}

#[test]
fn vec_non_empty_no_data() {
    let mut data = [0, 0, 0, 3, b'a', b'b', b'c'];

    for i in 0..7 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_vec().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        if i < 4 {
            assert_eq!(slice.len(), i);
        } else {
            assert_eq!(slice.len(), 0);
        }
    }
}

#[test]
fn vec_non_empty_complete() {
    let data = [0, 0, 0, 3, b'a', b'b', b'c'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_vec().unwrap(), [b'a', b'b', b'c']);
    assert_eq!(slice.len(), 0);
}

#[test]
fn vec_non_empty_remaining() {
    let data = [0, 0, 0, 3, b'a', b'b', b'c', b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_vec().unwrap(), [b'a', b'b', b'c']);
    assert_eq!(slice, [b'x']);
}

#[test]
fn revision_no_data() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let err = slice.read_revision().unwrap_err();
    assert_eq!(format!("{:?}", err), "NoData");
}

#[test]
fn revision_inval() {
    let data = [2];
    let mut slice: &[u8] = &data;

    let err = slice.read_revision().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalRevision)");
}

#[test]
fn revision_ok_complete() {
    let data = [1];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_revision().unwrap(), 1);
    assert_eq!(slice.len(), 0);
}

#[test]
fn revision_ok_remaining() {
    let data = [1, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_revision().unwrap(), 1);
    assert_eq!(slice, [b'x']);
}

#[test]
fn magic_no_data() {
    let mut data = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

    for i in 0..7 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_magic().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
        assert_eq!(slice.len(), 0);
    }
}

#[test]
fn magic_inval() {
    let data = [b'x', b'u', b't', b's', b'-', b'i', b'o'];
    let mut slice: &[u8] = &data;

    let err = slice.read_magic().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalMagic)");
}

#[test]
fn magic_ok_complete() {
    let data = [b'n', b'u', b't', b's', b'-', b'i', b'o'];
    let mut slice: &[u8] = &data;

    slice.read_magic().unwrap();
    assert_eq!(slice.len(), 0);
}

#[test]
fn magic_ok_remaining() {
    let data = [b'n', b'u', b't', b's', b'-', b'i', b'o', b'x'];
    let mut slice: &[u8] = &data;

    slice.read_magic().unwrap();
    assert_eq!(slice, [b'x']);
}

#[test]
fn cipher_no_data() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let err = slice.read_cipher().unwrap_err();
    assert_eq!(format!("{:?}", err), "NoData");
}

#[test]
fn cipher_inval() {
    let data = [2];
    let mut slice: &[u8] = &data;

    let err = slice.read_cipher().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalCipher)");
}

#[test]
fn cipher_none_complete() {
    let data = [0];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_cipher().unwrap(), Cipher::None);
    assert_eq!(slice.len(), 0);
}

#[test]
fn cipher_none_remaining() {
    let data = [0, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_cipher().unwrap(), Cipher::None);
    assert_eq!(slice, [b'x']);
}

#[test]
fn cipher_aes128_ctr_complete() {
    let data = [1];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_cipher().unwrap(), Cipher::Aes128Ctr);
    assert_eq!(slice.len(), 0);
}

#[test]
fn cipher_aes128_ctr_remaining() {
    let data = [1, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_cipher().unwrap(), Cipher::Aes128Ctr);
    assert_eq!(slice, [b'x']);
}

#[test]
fn digest_no_data() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let err = slice.read_digest().unwrap_err();
    assert_eq!(format!("{:?}", err), "NoData");
}

#[test]
fn digest_inval() {
    let data = [2];
    let mut slice: &[u8] = &data;

    let err = slice.read_digest().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDigest)");
}

#[test]
fn digest_none_complete() {
    let data = [0xff];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_digest().unwrap(), None);
    assert_eq!(slice.len(), 0);
}

#[test]
fn digest_none_remaining() {
    let data = [0xff, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_digest().unwrap(), None);
    assert_eq!(slice, [b'x']);
}

#[test]
fn digest_sha1_complete() {
    let data = [1];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_digest().unwrap(), Some(Digest::Sha1));
    assert_eq!(slice.len(), 0);
}

#[test]
fn digest_sha1_remaining() {
    let data = [1, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_digest().unwrap(), Some(Digest::Sha1));
    assert_eq!(slice, [b'x']);
}

#[test]
fn dtype_no_data() {
    let data = [0; 0];
    let mut slice: &[u8] = &data;

    let err = slice.read_dtype().unwrap_err();
    assert_eq!(format!("{:?}", err), "NoData");
}

#[test]
fn dtype_inval() {
    let data = [4];
    let mut slice: &[u8] = &data;

    let err = slice.read_dtype().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalDiskType)");
}

#[test]
fn dtype_fat_zero_complete() {
    let data = [0];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::FatZero);
    assert_eq!(slice.len(), 0);
}

#[test]
fn dtype_fat_zero_remaining() {
    let data = [0, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::FatZero);
    assert_eq!(slice, [b'x']);
}

#[test]
fn dtype_fat_random_complete() {
    let data = [1];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::FatRandom);
    assert_eq!(slice.len(), 0);
}

#[test]
fn dtype_fat_random_remaining() {
    let data = [1, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::FatRandom);
    assert_eq!(slice, [b'x']);
}

#[test]
fn dtype_thin_zero_complete() {
    let data = [2];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::ThinZero);
    assert_eq!(slice.len(), 0);
}

#[test]
fn dtype_thin_zero_remaining() {
    let data = [2, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::ThinZero);
    assert_eq!(slice, [b'x']);
}

#[test]
fn dtype_thin_random_complete() {
    let data = [3];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::ThinRandom);
    assert_eq!(slice.len(), 0);
}

#[test]
fn dtype_thin_random_remaining() {
    let data = [3, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_dtype().unwrap(), DiskType::ThinRandom);
    assert_eq!(slice, [b'x']);
}

#[test]
fn wrapping_key_inval() {
    let data = [2];
    let mut slice: &[u8] = &data;

    let err = slice.read_wrapping_key().unwrap_err();
    assert_eq!(format!("{:?}", err), "InvalHeader(InvalWrappingKey)");
}

#[test]
fn wrapping_key_pbkdf2_no_data() {
    let mut data = [1, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3];

    for i in 0..12 {
        let mut slice: &[u8] = data.get_mut(..i).unwrap();

        let err = slice.read_wrapping_key().unwrap_err();
        assert_eq!(format!("{:?}", err), "NoData");
    }
}

#[test]
fn wrapping_key_pbkdf2_complete() {
    let data = [1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3];
    let mut slice: &[u8] = &data;

    let wkey = slice.read_wrapping_key().unwrap();
    assert_eq!(
        wkey,
        Some(WrappingKeyData::Pbkdf2(Pbkdf2Data {
            iterations: 65536,
            salt: vec![1, 2, 3]
        }))
    );
    assert_eq!(slice.len(), 0);
}

#[test]
fn wrapping_key_pbkdf2_remaining() {
    let data = [
        1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, b'x',
    ];
    let mut slice: &[u8] = &data;

    let wkey = slice.read_wrapping_key().unwrap();
    assert_eq!(
        wkey,
        Some(WrappingKeyData::Pbkdf2(Pbkdf2Data {
            iterations: 65536,
            salt: vec![1, 2, 3]
        }))
    );
    assert_eq!(slice, [b'x']);
}

#[test]
fn wrapping_key_none_complete() {
    let data = [0xff];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_wrapping_key().unwrap(), None);
    assert_eq!(slice.len(), 0);
}

#[test]
fn wrapping_key_none_remaining() {
    let data = [0xff, b'x'];
    let mut slice: &[u8] = &data;

    assert_eq!(slice.read_wrapping_key().unwrap(), None);
    assert_eq!(slice, [b'x']);
}
