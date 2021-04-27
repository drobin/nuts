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

use crate::error::Error;
use crate::io::{FromBinary, IntoBinary};
use crate::types::Cipher;

#[test]
fn key_size_none() {
    assert_eq!(Cipher::None.key_size(), 0);
}

#[test]
fn key_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.key_size(), 16);
}

#[test]
fn key_size_aes128_gcm() {
    assert_eq!(Cipher::Aes128Gcm.key_size(), 16);
}

#[test]
fn iv_size_none() {
    assert_eq!(Cipher::None.iv_size(), 0);
}

#[test]
fn iv_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.iv_size(), 16);
}

#[test]
fn iv_size_aes128_gcm() {
    assert_eq!(Cipher::Aes128Gcm.iv_size(), 12);
}

#[test]
fn block_size_none() {
    assert_eq!(Cipher::None.block_size(), 1);
}

#[test]
fn block_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.block_size(), 1);
}

#[test]
fn block_size_aes128_gcm() {
    assert_eq!(Cipher::Aes128Gcm.block_size(), 1);
}

#[test]
fn tag_size_none() {
    assert_eq!(Cipher::None.tag_size(), 0);
}

#[test]
fn tag_size_aes128_ctr() {
    assert_eq!(Cipher::Aes128Ctr.tag_size(), 0);
}

#[test]
fn tag_size_aes128_gcm() {
    assert_eq!(Cipher::Aes128Gcm.tag_size(), 16);
}

#[test]
fn encrypt_none_empty() {
    let out = Cipher::None.encrypt(&[], &[], &[]).unwrap();
    assert!(out.is_empty());
}

#[test]
fn encrypt_none_non_empty() {
    let out = Cipher::None.encrypt(&[1, 2, 3], &[], &[]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn encrypt_aes128_ctr_key_too_short() {
    let key = [1; 15];
    let iv = [2; 16];

    if let Error::InvalArg(s) = Cipher::Aes128Ctr.encrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "key too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn encrypt_aes128_ctr_iv_too_short() {
    let key = [1; 16];
    let iv = [2; 15];

    if let Error::InvalArg(s) = Cipher::Aes128Ctr.encrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "iv too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn encrypt_aes128_ctr_empty() {
    let key = [1; 16];
    let iv = [2; 16];

    let out = Cipher::Aes128Ctr.encrypt(&[], &key, &iv).unwrap();
    assert!(out.is_empty());
}

#[test]
fn encrypt_aes128_ctr_non_empty() {
    let key = [1; 16];
    let iv = [2; 16];

    let out = Cipher::Aes128Ctr.encrypt(&[1, 2, 3], &key, &iv).unwrap();
    assert_eq!(out, [22, 212, 23]);
}

#[test]
fn encrypt_aes128_gcm_key_too_short() {
    let key = [1; 15];
    let iv = [2; 12];

    if let Error::InvalArg(s) = Cipher::Aes128Gcm.encrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "key too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn encrypt_aes128_gcm_iv_too_short() {
    let key = [1; 16];
    let iv = [2; 11];

    if let Error::InvalArg(s) = Cipher::Aes128Gcm.encrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "iv too short, at least 12 bytes needed but got 11");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn encrypt_aes128_gcm_empty() {
    let key = [1; 16];
    let iv = [2; 12];

    let out = Cipher::Aes128Gcm.encrypt(&[], &key, &iv).unwrap();
    assert_eq!(
        out,
        [128, 70, 184, 85, 160, 28, 19, 171, 139, 53, 126, 204, 49, 233, 152, 106]
    );
}

#[test]
fn encrypt_aes128_gcm_non_empty() {
    let key = [1; 16];
    let iv = [2; 12];

    let out = Cipher::Aes128Gcm.encrypt(&[1, 2, 3], &key, &iv).unwrap();
    assert_eq!(
        out,
        [82, 230, 19, 151, 34, 71, 241, 30, 13, 105, 152, 42, 49, 167, 188, 7, 114, 189, 125]
    );
}

#[test]
fn decrypt_none_empty() {
    let out = Cipher::None.decrypt(&[], &[], &[]).unwrap();
    assert!(out.is_empty());
}

#[test]
fn decrypt_none_non_empty() {
    let out = Cipher::None.decrypt(&[1, 2, 3], &[], &[]).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_aes128_ctr_key_too_short() {
    let key = [1; 15];
    let iv = [2; 16];

    if let Error::InvalArg(s) = Cipher::Aes128Ctr.decrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "key too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn decrypt_aes128_ctr_iv_too_short() {
    let key = [1; 16];
    let iv = [2; 15];

    if let Error::InvalArg(s) = Cipher::Aes128Ctr.decrypt(&[], &key, &iv).unwrap_err() {
        assert_eq!(s, "iv too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn decrypt_aes128_ctr_empty() {
    let key = [1; 16];
    let iv = [2; 16];

    let out = Cipher::Aes128Ctr.decrypt(&[], &key, &iv).unwrap();
    assert!(out.is_empty());
}

#[test]
fn decrypt_aes128_ctr_non_empty() {
    let key = [1; 16];
    let iv = [2; 16];

    let out = Cipher::Aes128Ctr
        .decrypt(&[22, 212, 23], &key, &iv)
        .unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_aes128_gcm_input_too_small() {
    let key = [1; 16];
    let iv = [2; 12];

    if let Error::InvalArg(s) = Cipher::Aes128Gcm.decrypt(&[3; 15], &key, &iv).unwrap_err() {
        assert_eq!(s, "input too small, length: 15, needed: 16");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn decrypt_aes128_gcm_key_too_short() {
    let key = [1; 15];
    let iv = [2; 12];

    if let Error::InvalArg(s) = Cipher::Aes128Gcm.decrypt(&[3; 16], &key, &iv).unwrap_err() {
        assert_eq!(s, "key too short, at least 16 bytes needed but got 15");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn decrypt_aes128_gcm_iv_too_short() {
    let key = [1; 16];
    let iv = [2; 11];

    if let Error::InvalArg(s) = Cipher::Aes128Gcm.decrypt(&[3; 16], &key, &iv).unwrap_err() {
        assert_eq!(s, "iv too short, at least 12 bytes needed but got 11");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn decrypt_aes128_gcm_empty() {
    let key = [1; 16];
    let iv = [2; 12];
    let input = [
        128, 70, 184, 85, 160, 28, 19, 171, 139, 53, 126, 204, 49, 233, 152, 106,
    ];

    let out = Cipher::Aes128Gcm.decrypt(&input, &key, &iv).unwrap();
    assert!(out.is_empty());
}

#[test]
fn decrypt_aes128_gcm_non_empty() {
    let key = [1; 16];
    let iv = [2; 12];
    let input = [
        82, 230, 19, 151, 34, 71, 241, 30, 13, 105, 152, 42, 49, 167, 188, 7, 114, 189, 125,
    ];

    let out = Cipher::Aes128Gcm.decrypt(&input, &key, &iv).unwrap();
    assert_eq!(out, [1, 2, 3]);
}

#[test]
fn decrypt_aes128_gcm_inval_tag() {
    let key = [1; 16];
    let iv = [2; 12];
    let input = [
        82, 230, 19, 151, 34, 71, 241, 30, 13, 105, 152, 42, 49, 167, 188, 7, 114, 189, b'x',
    ];

    match Cipher::Aes128Gcm.decrypt(&input, &key, &iv).unwrap_err() {
        Error::OpenSSL(_) => {}
        _ => panic!("invalid error"),
    }
}

#[test]
fn from_binary_none() {
    let mut c = Cursor::new(&[0]);
    assert_eq!(Cipher::from_binary(&mut c).unwrap(), Cipher::None);
}

#[test]
fn from_binary_aes128_ctr() {
    let mut c = Cursor::new(&[1]);
    assert_eq!(Cipher::from_binary(&mut c).unwrap(), Cipher::Aes128Ctr);
}

#[test]
fn from_binary_aes128_gcm() {
    let mut c = Cursor::new(&[2]);
    assert_eq!(Cipher::from_binary(&mut c).unwrap(), Cipher::Aes128Gcm);
}

#[test]
fn from_binary_inval() {
    let mut c = Cursor::new(&[3]);
    let err = Cipher::from_binary(&mut c).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(format!("{}", err), "invalid cipher detected");
}

#[test]
fn into_binary_none() {
    let mut c = Cursor::new(Vec::new());
    Cipher::None.into_binary(&mut c).unwrap();

    assert_eq!(c.into_inner(), [0]);
}

#[test]
fn into_binary_aes128_ctr() {
    let mut c = Cursor::new(Vec::new());
    Cipher::Aes128Ctr.into_binary(&mut c).unwrap();

    assert_eq!(c.into_inner(), [1]);
}

#[test]
fn into_binary_aes128_gcm() {
    let mut c = Cursor::new(Vec::new());
    Cipher::Aes128Gcm.into_binary(&mut c).unwrap();

    assert_eq!(c.into_inner(), [2]);
}
