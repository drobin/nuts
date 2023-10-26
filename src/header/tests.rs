// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use chrono::{TimeZone, Utc};
use nuts_bytes::{Reader, Writer};
use nuts_container::memory::MemoryBackend;

use crate::error::Error;
use crate::header::{Header, Magic};
use crate::tests::into_error;

#[test]
fn ser() {
    let header = Header {
        magic: Magic::new(),
        revision: 1,
        created: Utc.timestamp_millis_opt(2).unwrap(),
        modified: Utc.timestamp_millis_opt(3).unwrap(),
        nfiles: 4,
    };
    let mut writer = Writer::new(vec![]);

    writer.serialize(&header).unwrap();
    assert_eq!(
        writer.into_target(),
        [
            b'n', b'u', b't', b's', b'-', b'a', b'r', b'c', b'h', b'i', b'v', b'e', 0, 1, 0, 0, 0,
            0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4
        ]
    );
}

#[test]
fn de() {
    let mut reader = Reader::new(
        [
            b'n', b'u', b't', b's', b'-', b'a', b'r', b'c', b'h', b'i', b'v', b'e', 0, 1, 0, 0, 0,
            0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4,
        ]
        .as_slice(),
    );
    let header = reader.deserialize::<Header>().unwrap();

    assert_eq!(header.magic, b"nuts-archive");
    assert_eq!(header.revision, 1);
    assert_eq!(header.created.timestamp_millis(), 2);
    assert_eq!(header.modified.timestamp_millis(), 3);
    assert_eq!(header.nfiles, 4);
}

#[test]
fn de_inval_magic() {
    let mut reader = Reader::new(
        [
            b'x', b'u', b't', b's', b'-', b'a', b'r', b'c', b'h', b'i', b'v', b'e', 0, 1, 0, 0, 0,
            0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4,
        ]
        .as_slice(),
    );

    let err: Error<MemoryBackend> = reader.deserialize::<Header>().unwrap_err().into();
    let err = into_error!(err, Error::InvalidHeader);
    let err = into_error!(err, nuts_bytes::Error::Serde);
    assert_eq!(err, "invalid header-magic");
}

#[test]
fn inc_files() {
    let mut header = Header {
        magic: Magic::new(),
        revision: 1,
        created: Utc.timestamp_millis_opt(2).unwrap(),
        modified: Utc.timestamp_millis_opt(3).unwrap(),
        nfiles: 4,
    };

    header.inc_files();

    assert_eq!(header.nfiles, 5);
    assert_eq!(header.created.timestamp_millis(), 2);
    assert!(header.modified.timestamp_millis() > 3);
}
