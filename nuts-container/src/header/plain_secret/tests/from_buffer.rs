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

use nuts_memory::MemoryBackend;

use crate::header::plain_secret::tests::{rev0, rev1, rev1_no_top_id, rev2};
use crate::header::plain_secret::tests::{
    REV0, REV1, REV1_NO_TOP_ID, REV2_NONE, REV2_SID, REV2_TOP_ID,
};
use crate::header::plain_secret::PlainSecret;
use crate::header::HeaderError;

#[test]
fn rev0_ok() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev0(&mut &REV0[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev0(data) if data == rev0()));
}

#[test]
fn rev0_inval() {
    let mut vec = REV0.to_vec();
    vec[0] += 1;

    let err = PlainSecret::<MemoryBackend>::from_buffer_rev0(&mut &vec[..]).unwrap_err();

    assert!(matches!(err, HeaderError::WrongPassword));
}

#[test]
fn rev1_ok() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev1(&mut &REV1[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev1(data) if data == rev1()));
}

#[test]
fn rev1_without_top_id() {
    let out = PlainSecret::from_buffer_rev1(&mut &REV1_NO_TOP_ID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev1(data) if data == rev1_no_top_id()));
}

#[test]
fn rev1_inval() {
    let mut vec = REV1.to_vec();
    vec[0] += 1;

    match PlainSecret::<MemoryBackend>::from_buffer_rev1(&mut vec.as_slice()) {
        Ok(_) => panic!("unexpected result"),
        Err(err) => assert!(matches!(err, HeaderError::WrongPassword)),
    }
}

#[test]
fn rev2_sid() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut &REV2_SID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(Some(4711), None)));
}

#[test]
fn rev2_top_id() {
    let out = PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut &REV2_TOP_ID[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(None, Some("666"))));
}

#[test]
fn rev2_none() {
    let out = PlainSecret::from_buffer_rev2(&mut &REV2_NONE[..]).unwrap();

    assert!(matches!(out, PlainSecret::Rev2(data) if data == rev2(None, None)));
}

#[test]
fn rev2_inval() {
    let mut vec = REV2_NONE.to_vec();
    vec[0] += 1;

    match PlainSecret::<MemoryBackend>::from_buffer_rev2(&mut vec.as_slice()) {
        Ok(_) => panic!("unexpected result"),
        Err(err) => assert!(matches!(err, HeaderError::WrongPassword)),
    }
}
