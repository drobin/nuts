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

use nuts_bytes::{Reader, Writer};
use nuts_container::memory::{Id, MemoryBackend};

use crate::error::Error;
use crate::tests::{into_error, setup_container};
use crate::userdata::Userdata;

const USERDATA: [u8; 16] = [
    b'n', b'u', b't', b's', b'-', b'a', b'r', b'c', b'h', b'i', b'v', b'e', 0, 0, 0, 1,
];

#[test]
fn de_invalid_magic() {
    let mut bin = USERDATA;
    bin[0] += 1;

    let mut reader = Reader::new(&bin[..]);

    let err: Error<MemoryBackend> = reader
        .deserialize::<Userdata<MemoryBackend>>()
        .unwrap_err()
        .into();

    let err = into_error!(err, Error::InvalidUserdata);
    let msg = into_error!(err.unwrap(), nuts_bytes::Error::Serde);
    assert_eq!(msg, "invalid userdata-magic");
}

#[test]
fn de_ok() {
    let mut reader = Reader::new(&USERDATA[..]);

    let userdata = reader.deserialize::<Userdata<MemoryBackend>>().unwrap();
    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn ser_ok() {
    let id = "1".parse::<Id>().unwrap();
    let userdata = Userdata::<MemoryBackend>::new(id);

    let mut writer = Writer::new(vec![]);
    let n = writer.serialize(&userdata).unwrap();

    assert_eq!(n, 16);
    assert_eq!(writer.into_target(), USERDATA)
}

#[test]
fn create_userdata_unforced() {
    let mut container = setup_container();

    container.update_userdata(&[b'x'; 1]).unwrap();

    let err = Userdata::<MemoryBackend>::create(&mut container, false).unwrap_err();
    assert!(matches!(err, Error::OverwriteUserdata));
}

#[test]
fn create_userdata_forced() {
    let mut container = setup_container();

    container.update_userdata(&[b'x'; 1]).unwrap();

    let userdata = Userdata::<MemoryBackend>::create(&mut container, true).unwrap();

    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn create_no_userdata_unforced() {
    let mut container = setup_container();
    let userdata = Userdata::<MemoryBackend>::create(&mut container, false).unwrap();

    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn create_no_userdata_forced() {
    let mut container = setup_container();
    let userdata = Userdata::<MemoryBackend>::create(&mut container, true).unwrap();

    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn load_no_userdata() {
    let mut container = setup_container();

    let err = Userdata::<MemoryBackend>::load(&mut container).unwrap_err();
    let err = into_error!(err, Error::InvalidUserdata);
    assert!(err.is_none());
}

#[test]
fn load_invalid_userdata() {
    let mut container = setup_container();

    let userdata = [&[USERDATA[0] + 1], &USERDATA[1..]].concat();
    container.update_userdata(&userdata).unwrap();

    let err = Userdata::<MemoryBackend>::load(&mut container).unwrap_err();
    let err = into_error!(err, Error::InvalidUserdata);
    assert!(err.is_some());
}

#[test]
fn load_ok() {
    let mut container = setup_container();

    container.update_userdata(&USERDATA).unwrap();

    let userdata = Userdata::<MemoryBackend>::load(&mut container).unwrap();

    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn load_ok_attached() {
    let mut container = setup_container();

    let mut userdata = USERDATA.to_vec();
    userdata.push(b'x');
    container.update_userdata(&userdata).unwrap();

    let userdata = Userdata::<MemoryBackend>::load(&mut container).unwrap();

    assert_eq!(userdata.magic, b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}
