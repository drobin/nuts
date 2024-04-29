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

use nuts_bytes::{Reader, Writer};
use nuts_memory::MemoryBackend;

use crate::error::Error;
use crate::id::Id;
use crate::pager::Pager;
use crate::tests::{into_error, setup_container};
use crate::userdata::{Userdata, UserdataMagicError};

const USERDATA: [u8; 16] = [
    b'n', b'u', b't', b's', b'-', b'a', b'r', b'c', b'h', b'i', b'v', b'e', 0, 0, 0, 1,
];

#[test]
fn de_invalid_magic() {
    let mut bin = USERDATA;
    bin[0] += 1;

    let mut reader = Reader::new(&bin[..]);

    let err: Error<MemoryBackend> = reader.read::<Userdata<MemoryBackend>>().unwrap_err().into();

    let err = into_error!(err, Error::InvalidUserdata).unwrap();
    let err = into_error!(err, nuts_bytes::Error::Custom);
    assert!(err.is::<UserdataMagicError>());
}

#[test]
fn de_ok() {
    let mut reader = Reader::new(&USERDATA[..]);

    let userdata = reader.read::<Userdata<MemoryBackend>>().unwrap();
    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn ser_ok() {
    let id = "1".parse::<Id<MemoryBackend>>().unwrap();
    let userdata = Userdata::<MemoryBackend>::new(id);

    let mut writer = Writer::new(vec![]);
    let n = writer.write(&userdata).unwrap();

    assert_eq!(n, 16);
    assert_eq!(writer.into_target(), USERDATA)
}

#[test]
fn create_userdata_unforced() {
    let mut pager = Pager::new(setup_container());

    pager.update_userdata(&[b'x'; 1]).unwrap();

    let err = Userdata::<MemoryBackend>::create(&mut pager, false).unwrap_err();
    assert!(matches!(err, Error::OverwriteUserdata));
}

#[test]
fn create_userdata_forced() {
    let mut pager = Pager::new(setup_container());

    pager.update_userdata(&[b'x'; 1]).unwrap();

    let userdata = Userdata::<MemoryBackend>::create(&mut pager, true).unwrap();

    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn create_no_userdata_unforced() {
    let mut pager = Pager::new(setup_container());
    let userdata = Userdata::<MemoryBackend>::create(&mut pager, false).unwrap();

    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn create_no_userdata_forced() {
    let mut pager = Pager::new(setup_container());
    let userdata = Userdata::<MemoryBackend>::create(&mut pager, true).unwrap();

    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn load_no_userdata() {
    let mut pager = Pager::new(setup_container());

    let err = Userdata::<MemoryBackend>::load(&mut pager).unwrap_err();
    let err = into_error!(err, Error::InvalidUserdata);
    assert!(err.is_none());
}

#[test]
fn load_invalid_userdata() {
    let mut pager = Pager::new(setup_container());

    let userdata = [&[USERDATA[0] + 1], &USERDATA[1..]].concat();
    pager.update_userdata(&userdata).unwrap();

    let err = Userdata::<MemoryBackend>::load(&mut pager).unwrap_err();
    let err = into_error!(err, Error::InvalidUserdata);
    assert!(err.is_some());
}

#[test]
fn load_ok() {
    let mut pager = Pager::new(setup_container());

    pager.update_userdata(&USERDATA).unwrap();

    let userdata = Userdata::<MemoryBackend>::load(&mut pager).unwrap();

    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}

#[test]
fn load_ok_attached() {
    let mut pager = Pager::new(setup_container());

    let mut userdata = USERDATA.to_vec();
    userdata.push(b'x');
    pager.update_userdata(&userdata).unwrap();

    let userdata = Userdata::<MemoryBackend>::load(&mut pager).unwrap();

    assert_eq!(userdata.magic, *b"nuts-archive");
    assert_eq!(userdata.id.to_string(), "1");
}
