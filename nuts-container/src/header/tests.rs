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

use crate::cipher::Cipher;
use crate::header::plain_secret::{PlainRev0, PlainRev1, PlainRev2, PlainSecret};
use crate::header::{Header, HeaderError};
use crate::kdf::Kdf;
use crate::migrate::Migrator;
use crate::options::CreateOptionsBuilder;
use crate::password::PasswordStore;

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

const REV2: [u8; 56] = [
    b'n', b'u', b't', b's', b'-', b'i', b'o', // magic
    0, 0, 0, 2, // revision
    0, 0, 0, 0, // cipher
    0, 0, 0, 0, 0, 0, 0, 0, // iv
    0, 0, 0, 0, // kdf
    0, 0, 0, 0, 0, 0, 0, 21, // secret length
    0x91, 0xc0, 0xb2, 0xcf, 0x91, 0xc0, 0xb2, 0xcf, // secret: magics
    0,    // secret: key
    0,    // secret: iv
    0x00, 0x00, 0x02, 0x9a, // secret: sid
    4, 0x00, 0x00, 0x12, 0x67, // secret: top_id
    0, 0, // secret: settings
];

fn rev0_plain_secret(top_id: Option<&str>) -> PlainRev0<MemoryBackend> {
    PlainRev0 {
        magics: 0x91c0b2cf.into(),
        key: vec![].into(),
        iv: vec![].into(),
        userdata: vec![0x00, 0x00, 0x12, 0x67].into(),
        settings: Settings,
        sid: None,
        top_id: top_id.map(|id| id.parse().unwrap()),
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

fn rev2_plain_secret(top_id: Option<&str>) -> PlainRev2<MemoryBackend> {
    PlainRev2 {
        magics: 0x91c0b2cf.into(),
        key: vec![].into(),
        iv: vec![].into(),
        sid: None,
        top_id: top_id.map(|id| id.parse().unwrap()),
        settings: Settings,
    }
}

fn rev0(top_id: Option<&str>) -> PlainSecret<MemoryBackend> {
    PlainSecret::<MemoryBackend>::Rev0(rev0_plain_secret(top_id))
}

fn rev1(top_id: Option<&str>) -> PlainSecret<MemoryBackend> {
    PlainSecret::<MemoryBackend>::Rev1(rev1_plain_secret(top_id))
}

fn rev2(top_id: Option<&str>) -> PlainSecret<MemoryBackend> {
    PlainSecret::<MemoryBackend>::Rev2(rev2_plain_secret(top_id))
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

#[test]
fn create() {
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();
    let header = Header::<MemoryBackend>::create(&options, Settings).unwrap();

    assert_eq!(header.revision, 2);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, rev2(None));
}

#[test]
fn read_rev0() {
    let migrator = Migrator::default();
    let mut store = PasswordStore::new(None);

    let header = Header::<MemoryBackend>::read(&REV0, migrator, &mut store).unwrap();

    assert_eq!(header.revision, 0);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, rev0(None));
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
fn read_rev2() {
    let migrator = Migrator::default();
    let mut store = PasswordStore::new(None);

    let header = Header::<MemoryBackend>::read(&REV2, migrator, &mut store).unwrap();

    let expected_data = PlainSecret::Rev2(PlainRev2 {
        sid: Some(666),
        ..rev2_plain_secret(Some("4711"))
    });

    assert_eq!(header.revision, 2);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.kdf, Kdf::None);
    assert_eq!(header.data, expected_data);
}

#[test]
fn write_rev0() {
    let mut buf = [b'x'; REV0.len()];
    let mut store = PasswordStore::new(None);

    let header = header(rev0(None));

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
fn write_rev2() {
    let mut buf = [b'x'; REV2.len()];
    let mut store = PasswordStore::new(None);

    let header = header(PlainSecret::Rev2(PlainRev2 {
        sid: Some(666),
        ..rev2_plain_secret(Some("4711"))
    }));

    header.write(&mut buf, &mut store).unwrap();

    assert_eq!(buf, REV2);
}

#[test]
fn latest_revision_or_err_rev0() {
    let header = Header {
        revision: 0,
        ..header(rev0(None))
    };

    let err = header.latest_revision_or_err().unwrap_err();

    assert!(matches!(err, HeaderError::InvalidRevision(expected, got)
        if expected == 2 && got == 0))
}

#[test]
fn latest_revision_or_err_rev1() {
    let header = Header {
        revision: 1,
        ..header(rev1(None))
    };

    let err = header.latest_revision_or_err().unwrap_err();

    assert!(matches!(err, HeaderError::InvalidRevision(expected, got)
        if expected == 2 && got == 1))
}

#[test]
fn latest_revision_or_err_rev2() {
    let header = Header {
        revision: 2,
        ..header(rev2(None))
    };

    header.latest_revision_or_err().unwrap();
}

#[test]
fn settings_rev0() {
    let header = header(rev0(None));

    assert_eq!(header.settings().as_bytes(), Settings.as_bytes());
}

#[test]
fn settings_rev1() {
    let header = header(rev1(None));

    assert_eq!(header.settings().as_bytes(), Settings.as_bytes());
}

#[test]
fn settings_rev2() {
    let header = header(rev2(None));

    assert_eq!(header.settings().as_bytes(), Settings.as_bytes());
}

#[test]
fn key_rev0() {
    let rev0 = PlainRev0 {
        key: vec![1, 2, 3].into(),
        ..rev0_plain_secret(None)
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
fn key_rev2() {
    let rev2 = PlainRev2 {
        key: vec![1, 2, 3].into(),
        ..rev2_plain_secret(None)
    };
    let header = header(PlainSecret::Rev2(rev2));

    assert_eq!(header.key(), [1, 2, 3]);
}

#[test]
fn iv_rev0() {
    let rev0 = PlainRev0 {
        iv: vec![1, 2, 3].into(),
        ..rev0_plain_secret(None)
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
fn iv_rev2() {
    let rev2 = PlainRev2 {
        iv: vec![1, 2, 3].into(),
        ..rev2_plain_secret(None)
    };
    let header = header(PlainSecret::Rev2(rev2));

    assert_eq!(header.iv(), [1, 2, 3]);
}

#[test]
fn can_accept_sid_rev0_none() {
    let rev0 = rev0_plain_secret(None);
    let header = header(PlainSecret::Rev0(rev0));

    let err = header.can_accept_sid(666).unwrap_err();

    assert!(matches!(err, HeaderError::UnexpectedSid(expected, got)
        if expected == 666 && got.is_none()));
}

#[test]
fn can_accept_sid_rev0_some_eq() {
    let rev0 = PlainRev0 {
        sid: Some(666),
        ..rev0_plain_secret(None)
    };
    let header = header(PlainSecret::Rev0(rev0));

    header.can_accept_sid(666).unwrap();
}

#[test]
fn can_accept_sid_rev0_some_neq() {
    let rev0 = PlainRev0 {
        sid: Some(4711),
        ..rev0_plain_secret(None)
    };
    let header = header(PlainSecret::Rev0(rev0));

    let err = header.can_accept_sid(666).unwrap_err();

    assert!(matches!(err, HeaderError::UnexpectedSid(expected, got)
        if expected == 666 && got == Some(4711)));
}

#[test]
fn can_accept_sid_rev1() {
    let header = header(rev1(None));

    header.can_accept_sid(666).unwrap();
}

#[test]
fn can_accept_sid_rev2_none() {
    let rev2 = rev2_plain_secret(None);
    let header = header(PlainSecret::Rev2(rev2));

    let err = header.can_accept_sid(666).unwrap_err();

    assert!(matches!(err, HeaderError::UnexpectedSid(expected, got)
        if expected == 666 && got.is_none()));
}

#[test]
fn can_accept_sid_rev2_some_eq() {
    let rev2 = PlainRev2 {
        sid: Some(666),
        ..rev2_plain_secret(None)
    };
    let header = header(PlainSecret::Rev2(rev2));

    header.can_accept_sid(666).unwrap();
}

#[test]
fn can_accept_sid_rev2_some_neq() {
    let rev2 = PlainRev2 {
        sid: Some(4711),
        ..rev2_plain_secret(None)
    };
    let header = header(PlainSecret::Rev2(rev2));

    let err = header.can_accept_sid(666).unwrap_err();

    assert!(matches!(err, HeaderError::UnexpectedSid(expected, got)
        if expected == 666 && got == Some(4711)));
}

#[test]
#[should_panic(expected = "storing a sid into a rev0 header is not supported")]
fn set_sid_rev0() {
    header(rev0(None)).set_sid(666).unwrap();
}

#[test]
#[should_panic(expected = "storing a sid into a rev0 header is not supported")]
fn set_sid_rev0_inval() {
    header(rev0(None)).set_sid(0).unwrap();
}

#[test]
#[should_panic(expected = "storing a sid into a rev1 header is not supported")]
fn set_sid_rev1() {
    header(rev1(None)).set_sid(666).unwrap();
}

#[test]
#[should_panic(expected = "storing a sid into a rev1 header is not supported")]
fn set_sid_rev1_inval() {
    header(rev1(None)).set_sid(0).unwrap();
}

#[test]
fn set_sid_rev2() {
    let mut header = header(rev2(None));

    header.set_sid(666).unwrap();

    assert!(matches!(header.data, PlainSecret::Rev2(rev2) if rev2.sid == Some(666)));
}

#[test]
fn set_sid_rev2_inval() {
    let mut header = header(rev2(None));
    let err = header.set_sid(0).unwrap_err();

    assert!(matches!(err, HeaderError::InvalidSid));
}

#[test]
fn top_id_rev0_none() {
    let header = header(rev0(None));

    assert!(header.top_id().is_none());
}

#[test]
fn top_id_rev0_some() {
    let header = header(rev0(Some("4711")));
    let top_id = header.top_id().unwrap();

    assert_eq!(top_id.to_string(), "4711");
}

#[test]
fn top_id_rev1_none() {
    let header = header(rev1(None));

    assert!(header.top_id().is_none());
}

#[test]
fn top_id_rev1_some() {
    let header = header(rev1(Some("4711")));
    let top_id = header.top_id().unwrap();

    assert_eq!(top_id.to_string(), "4711");
}

#[test]
fn top_id_rev2_none() {
    let header = header(rev2(None));

    assert!(header.top_id().is_none());
}

#[test]
fn top_id_rev2_some() {
    let header = header(rev2(Some("4711")));
    let top_id = header.top_id().unwrap();

    assert_eq!(top_id.to_string(), "4711");
}

#[test]
#[should_panic(expected = "storing a top-id into a rev0 header is not supported")]
fn set_top_id_rev0() {
    header(rev0(None)).set_top_id("4711".parse().unwrap());
}

#[test]
#[should_panic(expected = "storing a top-id into a rev1 header is not supported")]
fn set_top_id_rev1() {
    header(rev1(None)).set_top_id("4711".parse().unwrap());
}

#[test]
fn set_top_id_rev2() {
    let mut header = header(rev2(None));

    header.set_top_id("4711".parse().unwrap());

    assert_eq!(header.top_id().unwrap().to_string(), "4711");
}
