// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use nuts_memory::{MemoryBackend, Settings};

use crate::buffer::ToBuffer;
use crate::header::plain_secret::{Magics, PlainRev0, PlainRev1, PlainRev2, PlainSecret};
use crate::header::HeaderError;
use crate::migrate::{Migration, MigrationError, Migrator};

const REV0: [u8; 49] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    0, 0, 0, 0, 0, 0, 0, 2, 1, 2, // key
    0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 5, // iv
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x12, 0x67, // userdata
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // settings (empty)
];

const REV1: [u8; 22] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    2, 1, 2, // key
    3, 3, 4, 5, // iv
    4, 0, 0, 2, 154, // top-id
    0, 0, // settings
];

const REV1_NO_TOP_ID: [u8; 18] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    2, 1, 2, // key
    3, 3, 4, 5, // iv
    0, // top-id
    0, 0, // settings
];

const REV2_SID: [u8; 22] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    2, 1, 2, // key
    3, 3, 4, 5, // iv
    0, 0, 0x12, 0x67, // sid
    0,    // top-id
    0, 0, // settings
];

const REV2_TOP_ID: [u8; 26] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    2, 1, 2, // key
    3, 3, 4, 5, // iv
    0, 0, 0, 0, // sid
    4, 0, 0, 2, 154, // top-id
    0, 0, // settings
];

const REV2_NONE: [u8; 22] = [
    0x00, 0x00, 0x12, 0x67, // magic1
    0x00, 0x00, 0x12, 0x67, // magic2
    2, 1, 2, // key
    3, 3, 4, 5, // iv
    0, 0, 0, 0, // sid
    0, // top-id
    0, 0, // settings
];

fn rev0() -> PlainRev0<MemoryBackend> {
    PlainRev0 {
        magics: Magics([4711, 4711]),
        key: vec![1, 2].into(),
        iv: vec![3, 4, 5].into(),
        userdata: vec![0x00, 0x00, 0x12, 0x67].into(),
        settings: Settings,
        sid: None,
        top_id: None,
    }
}

fn rev1() -> PlainRev1<MemoryBackend> {
    PlainRev1 {
        magics: Magics([4711, 4711]),
        key: vec![1, 2].into(),
        iv: vec![3, 4, 5].into(),
        top_id: Some("666".parse().unwrap()),
        settings: Settings,
    }
}

fn rev1_no_top_id() -> PlainRev1<MemoryBackend> {
    PlainRev1 {
        top_id: None,
        ..rev1()
    }
}

fn rev2(sid: Option<u32>, top_id: Option<&str>) -> PlainRev2<MemoryBackend> {
    PlainRev2 {
        magics: Magics([4711, 4711]),
        key: vec![1, 2].into(),
        iv: vec![3, 4, 5].into(),
        sid,
        top_id: top_id.map(|id| id.parse().unwrap()),
        settings: Settings,
    }
}

struct SampleMigration;

impl Migration for SampleMigration {
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<(u32, Vec<u8>), String> {
        assert_eq!(userdata, [0x00, 0x00, 0x12, 0x67]);
        Ok((666, userdata.to_vec()))
    }
}

struct ErrMigration;

impl Migration for ErrMigration {
    fn migrate_rev0(&self, _userdata: &[u8]) -> Result<(u32, Vec<u8>), String> {
        Err("foo".to_string())
    }
}

#[test]
fn create_latest() {
    let (revision, plain_secret) =
        PlainSecret::<MemoryBackend>::create_latest(vec![1].into(), vec![2, 3].into(), Settings)
            .unwrap();

    let expected = PlainRev2::<MemoryBackend> {
        magics: Magics([0x91C0B2CF; 2]),
        key: vec![1].into(),
        iv: vec![2, 3].into(),
        sid: None,
        top_id: None,
        settings: Settings,
    };

    assert_eq!(revision, 2);
    assert!(matches!(plain_secret, PlainSecret::Rev2(data) if data == expected));
}

#[test]
fn from_buffer_rev0() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev0(&mut &REV0[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev0(data) if data == rev0()));
}

#[test]
fn from_buffer_rev0_inval() {
    let mut vec = REV0.to_vec();
    vec[0] += 1;

    let err = PlainSecret::<MemoryBackend>::from_buffer_rev0(&mut &vec[..]).unwrap_err();

    assert!(matches!(err, HeaderError::WrongPassword));
}

#[test]
fn from_buffer_rev1() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev1(&mut &REV1[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev1(data) if data == rev1()));
}

#[test]
fn from_buffer_rev1_no_top_id() {
    let out = PlainSecret::from_buffer_rev1(&mut &REV1_NO_TOP_ID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev1(data) if data == rev1_no_top_id()));
}

#[test]
fn from_buffer_rev1_inval() {
    let mut vec = REV1.to_vec();
    vec[0] += 1;

    match PlainSecret::<MemoryBackend>::from_buffer_rev1(&mut vec.as_slice()) {
        Ok(_) => panic!("unexpected result"),
        Err(err) => assert!(matches!(err, HeaderError::WrongPassword)),
    }
}

#[test]
fn from_buffer_rev2_sid() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut &REV2_SID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(Some(4711), None)));
}

#[test]
fn from_buffer_rev2_top_id() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut &REV2_TOP_ID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(None, Some("666"))));
}

#[test]
fn from_buffer_rev2_none() {
    let out = PlainSecret::from_buffer_rev2(&mut &REV2_NONE[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(None, None)));
}

#[test]
fn from_buffer_rev2_inval() {
    let mut vec = REV2_NONE.to_vec();
    vec[0] += 1;

    match PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut vec.as_slice()) {
        Ok(_) => panic!("unexpected result"),
        Err(err) => assert!(matches!(err, HeaderError::WrongPassword)),
    }
}

#[test]
fn to_buffer_rev0() {
    let mut buf = vec![];

    PlainSecret::Rev0(rev0()).to_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV0);
}

#[test]
fn to_buffer_rev1() {
    let mut buf = vec![];

    PlainSecret::Rev1(rev1()).to_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV1);
}

#[test]
fn to_buffer_rev1_no_top_id() {
    let mut buf = vec![];

    PlainSecret::Rev1(rev1_no_top_id())
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV1_NO_TOP_ID);
}

#[test]
fn to_buffer_rev2_sid() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(Some(4711), None))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_SID);
}

#[test]
fn to_buffer_rev2_top_id() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(None, Some("666")))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_TOP_ID);
}

#[test]
fn to_buffer_rev2_none() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(None, None))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_NONE);
}

#[test]
fn migrate_rev0_no_migrator() {
    let migrator = Migrator::default();
    let mut rev0 = rev0();

    rev0.migrate(&migrator).unwrap();

    assert!(rev0.top_id.is_none());
}

#[test]
fn migrate_rev0_migrated() {
    let migrator = Migrator::default().with_migration(SampleMigration);
    let mut rev0 = rev0();

    rev0.migrate(&migrator).unwrap();

    assert_eq!(rev0.top_id.unwrap().to_string(), "4711");
}

#[test]
fn migrate_rev0_inval_migration() {
    // FIXME Id of MemoryBackend is always valid
}

#[test]
fn migrate_rev0_err_migration() {
    let migrator = Migrator::default().with_migration(ErrMigration);
    let mut rev0 = rev0();

    let err = rev0.migrate(&migrator).unwrap_err();

    assert!(matches!(err, HeaderError::Migration(cause)
        if matches!(cause, MigrationError::Rev0(ref msg) if msg == "foo")));
}
