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

use crate::error::Error;
use crate::header::ser::HeaderWriter;
use crate::types::{Cipher, Digest, DiskType, WrappingKey};

#[test]
fn revision_no_space() {
    let mut data = [b'x'; 0];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_revision(1).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn revision_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_revision(1).unwrap();
    assert_eq!(data, [1, b'x']);
}

#[test]
fn magic_no_space() {
    let mut data = [b'x'; 6];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_magic().unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn magic_ok() {
    let mut data = [b'x'; 8];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_magic().unwrap();
    assert_eq!(data, [b'n', b'u', b't', b's', b'-', b'i', b'o', b'x']);
}

#[test]
fn cipher_no_space() {
    let mut data = [b'x'; 0];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_cipher(Cipher::None).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn cipher_none_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_cipher(Cipher::None).unwrap();
    assert_eq!(data, [0, b'x']);
}

#[test]
fn cipher_aes128_ctr_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_cipher(Cipher::Aes128Ctr).unwrap();
    assert_eq!(data, [1, b'x']);
}

#[test]
fn digest_no_space() {
    let mut data = [b'x'; 0];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_digest(None).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn digest_none_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_digest(None).unwrap();
    assert_eq!(data, [0xff, b'x']);
}

#[test]
fn digest_sha1_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_digest(Some(Digest::Sha1)).unwrap();
    assert_eq!(data, [0x1, b'x']);
}

#[test]
fn dtype_no_space() {
    let mut data = [b'x'; 0];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_dtype(DiskType::FatZero).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn dtype_fat_zero_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_dtype(DiskType::FatZero).unwrap();
    assert_eq!(data, [0, b'x']);
}

#[test]
fn dtype_fat_random_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_dtype(DiskType::FatRandom).unwrap();
    assert_eq!(data, [1, b'x']);
}

#[test]
fn dtype_thin_zero_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_dtype(DiskType::ThinZero).unwrap();
    assert_eq!(data, [2, b'x']);
}

#[test]
fn dtype_thin_random_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_dtype(DiskType::ThinRandom).unwrap();
    assert_eq!(data, [3, b'x']);
}

#[test]
fn wrapping_key_none_no_space() {
    let mut data = [b'x'; 0];
    let mut writer = HeaderWriter::new(&mut data);

    if let Error::IoError(err) = writer.write_wrapping_key(None).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn wrapping_key_none_ok() {
    let mut data = [b'x'; 2];
    let mut writer = HeaderWriter::new(&mut data);

    writer.write_wrapping_key(None).unwrap();
    assert_eq!(data, [0xff, b'x']);
}

#[test]
fn wrapping_key_pbkdf2_no_space() {
    let mut data = [b'x'; 11];
    let mut writer = HeaderWriter::new(&mut data);
    let wkey = WrappingKey::Pbkdf2 {
        iterations: 65536,
        salt: vec![1, 2, 3],
    };

    if let Error::IoError(err) = writer.write_wrapping_key(Some(&wkey)).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::WriteZero);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn wrapping_key_pbkdf2_ok() {
    let mut data = [b'x'; 13];
    let mut writer = HeaderWriter::new(&mut data);
    let wkey = WrappingKey::Pbkdf2 {
        iterations: 65536,
        salt: vec![1, 2, 3],
    };

    writer.write_wrapping_key(Some(&wkey)).unwrap();
    assert_eq!(
        data,
        [1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, b'x']
    );
}
