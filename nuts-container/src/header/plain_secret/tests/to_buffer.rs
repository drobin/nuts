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

use crate::buffer::ToBuffer;
use crate::header::plain_secret::tests::{rev0, rev1, rev1_no_top_id, rev2};
use crate::header::plain_secret::tests::{
    REV0, REV1, REV1_NO_TOP_ID, REV2_NONE, REV2_SID, REV2_TOP_ID,
};
use crate::header::plain_secret::PlainSecret;

#[test]
fn rev0_ok() {
    let mut buf = vec![];

    PlainSecret::Rev0(rev0()).to_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV0);
}

#[test]
fn rev1_ok() {
    let mut buf = vec![];

    PlainSecret::Rev1(rev1()).to_buffer(&mut buf).unwrap();
    assert_eq!(buf, REV1);
}

#[test]
fn rev1_without_top_id() {
    let mut buf = vec![];

    PlainSecret::Rev1(rev1_no_top_id())
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV1_NO_TOP_ID);
}

#[test]
fn rev2_sid() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(Some(4711), None))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_SID);
}

#[test]
fn rev2_top_id() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(None, Some("666")))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_TOP_ID);
}

#[test]
fn rev2_none() {
    let mut buf = vec![];

    PlainSecret::Rev2(rev2(None, None))
        .to_buffer(&mut buf)
        .unwrap();
    assert_eq!(buf, REV2_NONE);
}
