// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use nuts_backend::HEADER_MAX_SIZE;
use nuts_memory::{MemoryBackend, Settings};

use crate::cipher::Cipher;
use crate::header::Header;
use crate::kdf::Kdf;
use crate::migrate::Migration;
use crate::options::OpenOptionsBuilder;
use crate::password::PasswordStore;

const REV0: [u8; 77] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0, 0, 0, 0, // revision
    0, 0, 0, 0, // cipher
    0, 0, 0, 0, 0, 0, 0, 0, // iv
    0, 0, 0, 0, // kdf
    0, 0, 0, 0, 0, 0, 0, 42, // secret length
    0x91, 0xc0, 0xb2, 0xcf, 0x91, 0xc0, 0xb2, 0xcf, // secret: magics
    0, 0, 0, 0, 0, 0, 0, 0, // secret: key
    0, 0, 0, 0, 0, 0, 0, 0, // secret: iv
    0, 0, 0, 0, 0, 0, 0, 2, 0x12, 0x67, // secret: userdata
    0, 0, 0, 0, 0, 0, 0, 0, // secret: settings
];

const REV1: [u8; 50] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0, 0, 0, 1, // revision
    0, 0, 0, 0, // cipher
    0, 0, 0, 0, 0, 0, 0, 0, // iv
    0, 0, 0, 0, // kdf
    0, 0, 0, 0, 0, 0, 0, 15, // secret length
    0x91, 0xc0, 0xb2, 0xcf, 0x91, 0xc0, 0xb2, 0xcf, // secret: magics
    0,    // secret: key
    0,    // secret: iv
    2, 0x12, 0x67, // secret: top_id
    0, 0, // secret: settings
];

struct SampleMigration;

impl Migration for SampleMigration {
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<Vec<u8>, String> {
        assert_eq!(userdata, [0x00, 0x00, 0x12, 0x67]);
        Ok(userdata.to_vec())
    }
}

#[test]
fn read_rev0() {
    let options = OpenOptionsBuilder::new()
        .with_migrator(SampleMigration)
        .build::<MemoryBackend>()
        .unwrap();
    let mut store = PasswordStore::new(None);

    let (header, _) = Header::read::<MemoryBackend>(&REV0, options, &mut store).unwrap();

    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert!(header.key.is_empty());
    assert!(header.iv.is_empty());
    assert_eq!(*header.top_id.unwrap(), [0x12, 0x67]);
}

#[test]
fn read_rev0_migration_not_required() {
    let options = OpenOptionsBuilder::new()
        .with_migration_required(false)
        .build::<MemoryBackend>()
        .unwrap();
    let mut store = PasswordStore::new(None);

    let (header, _) = Header::read::<MemoryBackend>(&REV0, options, &mut store).unwrap();

    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert!(header.key.is_empty());
    assert!(header.iv.is_empty());
    assert!(header.top_id.is_none());
}

#[test]
fn read_rev1() {
    let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();
    let mut store = PasswordStore::new(None);

    let (header, _) = Header::read::<MemoryBackend>(&REV1, options, &mut store).unwrap();

    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert!(header.key.is_empty());
    assert!(header.iv.is_empty());
    assert_eq!(*header.top_id.unwrap(), [0x12, 0x67]);
}

#[test]
fn write() {
    let mut buf = [b'x'; HEADER_MAX_SIZE];
    let mut store = PasswordStore::new(None);
    let header = Header {
        cipher: Cipher::None,
        kdf: Kdf::None,
        key: vec![].into(),
        iv: vec![].into(),
        top_id: Some(vec![0x12, 0x67].into()),
    };

    header
        .write::<MemoryBackend>(Settings, &mut buf, &mut store)
        .unwrap();

    assert_eq!(&buf[..REV1.len()], REV1);
    assert_eq!(buf[REV1.len()..], [b'x'; HEADER_MAX_SIZE - REV1.len()]);
}
