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

use nuts_backend::Binary;
use nuts_memory::{MemoryBackend, Settings};
use std::borrow::Cow;

use crate::cipher::Cipher;
use crate::header::plain_secret::{PlainRev0, PlainRev1, PlainSecret};
use crate::header::Header;
use crate::kdf::Kdf;
use crate::migrate::{Migration, Migrator};
use crate::options::CreateOptionsBuilder;
use crate::password::PasswordStore;
use crate::{HeaderError, MigrationError};

const REV0: [u8; 79] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0, 0, 0, 0, // revision
    0, 0, 0, 0, // cipher
    0, 0, 0, 0, 0, 0, 0, 0, // iv
    0, 0, 0, 0, // kdf
    0, 0, 0, 0, 0, 0, 0, 44, // secret length
    0x91, 0xc0, 0xb2, 0xcf, 0x91, 0xc0, 0xb2, 0xcf, // secret: magics
    0, 0, 0, 0, 0, 0, 0, 0, // secret: key
    0, 0, 0, 0, 0, 0, 0, 0, // secret: iv
    0, 0, 0, 0, 0, 0, 0, 4, 0x00, 0x00, 0x12, 0x67, // secret: userdata
    0, 0, 0, 0, 0, 0, 0, 0, // secret: settings
];

const REV1: [u8; 52] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0, 0, 0, 1, // revision
    0, 0, 0, 0, // cipher
    0, 0, 0, 0, 0, 0, 0, 0, // iv
    0, 0, 0, 0, // kdf
    0, 0, 0, 0, 0, 0, 0, 17, // secret length
    0x91, 0xc0, 0xb2, 0xcf, 0x91, 0xc0, 0xb2, 0xcf, // secret: magics
    0,    // secret: key
    0,    // secret: iv
    4, 0x00, 0x00, 0x12, 0x67, // secret: top_id
    0, 0, // secret: settings
];

fn rev0_plain_secret() -> PlainRev0<MemoryBackend> {
    PlainRev0 {
        magics: 0x91c0b2cf.into(),
        key: vec![].into(),
        iv: vec![].into(),
        userdata: vec![0x00, 0x00, 0x12, 0x67].into(),
        settings: Settings,
    }
}

fn rev1_plain_secret(top_id: Option<&str>) -> PlainRev1<MemoryBackend> {
    PlainRev1 {
        magics: 0x91c0b2cf.into(),
        key: vec![].into(),
        iv: vec![].into(),
        top_id: top_id.map(|id| id.parse().unwrap()),
        settings: Settings,
    }
}

fn rev0() -> PlainSecret<MemoryBackend> {
    PlainSecret::<MemoryBackend>::Rev0(rev0_plain_secret())
}

fn rev1(top_id: Option<&str>) -> PlainSecret<MemoryBackend> {
    PlainSecret::<MemoryBackend>::Rev1(rev1_plain_secret(top_id))
}

fn header(data: PlainSecret<MemoryBackend>) -> Header<'static, MemoryBackend> {
    Header::<MemoryBackend> {
        revision: 1,
        migrator: Migrator::default(),
        cipher: Cipher::None,
        kdf: Kdf::None,
        data,
    }
}

struct SampleMigration;

impl Migration for SampleMigration {
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<Vec<u8>, String> {
        assert_eq!(userdata, [0x00, 0x00, 0x12, 0x67]);
        Ok(userdata.to_vec())
    }
}

struct ErrMigration;

impl Migration for ErrMigration {
    fn migrate_rev0(&self, _userdata: &[u8]) -> Result<Vec<u8>, String> {
        Err("foo".to_string())
    }
}

#[test]
fn create() {
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();
    let header = Header::<MemoryBackend>::create(&options, Settings).unwrap();

    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, rev1(None));
}

#[test]
fn read_rev0() {
    let migrator = Migrator::default();
    let mut store = PasswordStore::new(None);

    let header = Header::<MemoryBackend>::read(&REV0, migrator, &mut store).unwrap();

    assert_eq!(header.revision, 0);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, rev0());
}

#[test]
fn read_rev1() {
    let migrator = Migrator::default();
    let mut store = PasswordStore::new(None);

    let header = Header::<MemoryBackend>::read(&REV1, migrator, &mut store).unwrap();

    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, rev1(Some("4711")));
}

#[test]
fn write_rev0() {
    let mut buf = [b'x'; REV0.len()];
    let mut store = PasswordStore::new(None);

    let header = header(rev0());

    header.write(&mut buf, &mut store).unwrap();

    assert_eq!(buf, REV0);
}

#[test]
fn write_rev1() {
    let mut buf = [b'x'; REV1.len()];
    let mut store = PasswordStore::new(None);

    let header = header(rev1(Some("4711")));

    header.write(&mut buf, &mut store).unwrap();

    assert_eq!(buf, REV1);
}

#[test]
fn settings_rev0() {
    let header = header(rev0());

    assert_eq!(header.settings().as_bytes(), Settings.as_bytes());
}

#[test]
fn settings_rev1() {
    let header = header(rev1(None));

    assert_eq!(header.settings().as_bytes(), Settings.as_bytes());
}

#[test]
fn key_rev0() {
    let rev0 = PlainRev0 {
        key: vec![1, 2, 3].into(),
        ..rev0_plain_secret()
    };
    let header = header(PlainSecret::Rev0(rev0));

    assert_eq!(header.key(), [1, 2, 3]);
}

#[test]
fn key_rev1() {
    let rev1 = PlainRev1 {
        key: vec![1, 2, 3].into(),
        ..rev1_plain_secret(None)
    };
    let header = header(PlainSecret::Rev1(rev1));

    assert_eq!(header.key(), [1, 2, 3]);
}

#[test]
fn iv_rev0() {
    let rev0 = PlainRev0 {
        iv: vec![1, 2, 3].into(),
        ..rev0_plain_secret()
    };
    let header = header(PlainSecret::Rev0(rev0));

    assert_eq!(header.iv(), [1, 2, 3]);
}

#[test]
fn iv_rev1() {
    let rev1 = PlainRev1 {
        iv: vec![1, 2, 3].into(),
        ..rev1_plain_secret(None)
    };
    let header = header(PlainSecret::Rev1(rev1));

    assert_eq!(header.iv(), [1, 2, 3]);
}

#[test]
fn top_id_rev0_no_migrator() {
    let header = header(rev0());

    assert!(header.top_id().unwrap().is_none());
}

#[test]
fn top_id_rev0_migrated() {
    let mut header = header(rev0());

    header.migrator = Migrator::default().with_migration(SampleMigration);

    let top_id = header.top_id().unwrap().unwrap();

    assert!(matches!(top_id, Cow::Owned(_)));
    assert_eq!(top_id.as_ref().to_string(), "4711");
}

#[test]
fn top_id_rev0_inval() {
    // FIXME Id of MemoryBackend is always valid
}

#[test]
fn top_id_rev0_err() {
    let mut header = header(rev0());

    header.migrator = Migrator::default().with_migration(ErrMigration);

    let err = header.top_id().unwrap_err();

    assert!(matches!(err, HeaderError::Migration(cause)
        if matches!(cause, MigrationError::Rev0(ref msg) if msg == "foo")));
}

#[test]
fn top_id_rev1_none() {
    let header = header(rev1(None));

    assert!(header.top_id().unwrap().is_none());
}

#[test]
fn top_id_rev1_some() {
    let header = header(rev1(Some("4711")));
    let top_id = header.top_id().unwrap().unwrap();

    assert!(matches!(top_id, Cow::Borrowed(_)));
    assert_eq!(top_id.as_ref().to_string(), "4711");
}

#[test]
#[should_panic(expected = "storing a top-id into a rev0 header is not supported")]
fn set_top_id_rev0() {
    header(rev0()).set_top_id("4711".parse().unwrap());
}

#[test]
fn set_top_id_rev1() {
    let mut header = header(rev1(None));

    header.set_top_id("4711".parse().unwrap());

    assert_eq!(header.top_id().unwrap().unwrap().to_string(), "4711");
}
