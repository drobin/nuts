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

use nuts_memory::Settings;

use crate::header::plain_secret::tests::{rev0, rev1, rev2};
use crate::header::plain_secret::{PlainRev0, PlainRev1, PlainSecret};

#[test]
fn rev0_no_top_id() {
    let mut plain_secret = PlainSecret::Rev0(PlainRev0 {
        sid: Some(666),
        ..rev0()
    });

    assert!(plain_secret.convert_to_latest(666));

    assert!(matches!(plain_secret, PlainSecret::Rev2(rev2)
        if rev2.magics == 4711.into() &&
           *rev2.key == [1, 2] &&
           *rev2.iv == [3, 4, 5] &&
          rev2.sid == Some(666) &&
          rev2.top_id.is_none() &&
          rev2.settings == Settings));
}

#[test]
fn rev0_top_id() {
    let mut plain_secret = PlainSecret::Rev0(PlainRev0 {
        sid: Some(666),
        top_id: Some("4711".parse().unwrap()),
        ..rev0()
    });

    assert!(plain_secret.convert_to_latest(666));

    assert!(matches!(plain_secret, PlainSecret::Rev2(rev2)
        if rev2.magics == 4711.into() &&
           *rev2.key == [1, 2] &&
           *rev2.iv == [3, 4, 5] &&
          rev2.sid == Some(666) &&
          rev2.top_id.unwrap().to_string() == "4711" &&
          rev2.settings == Settings));
}

#[test]
fn rev1_no_top_id() {
    let mut plain_secret = PlainSecret::Rev1(PlainRev1 {
        top_id: None,
        ..rev1()
    });

    assert!(plain_secret.convert_to_latest(666));

    assert!(matches!(plain_secret, PlainSecret::Rev2(rev2)
        if rev2.magics == 4711.into() &&
           *rev2.key == [1, 2] &&
           *rev2.iv == [3, 4, 5] &&
          rev2.sid == Some(666) &&
          rev2.top_id.is_none() &&
          rev2.settings == Settings));
}

#[test]
fn rev1_top_id() {
    let mut plain_secret = PlainSecret::Rev1(rev1());

    assert!(plain_secret.convert_to_latest(666));

    assert!(matches!(plain_secret, PlainSecret::Rev2(rev2)
        if rev2.magics == 4711.into() &&
           *rev2.key == [1, 2] &&
           *rev2.iv == [3, 4, 5] &&
          rev2.sid == Some(666) &&
          rev2.top_id.unwrap().to_string() == "666" &&
          rev2.settings == Settings));
}

#[test]
fn rev2_not_modified() {
    let mut plain_secret = PlainSecret::Rev2(rev2(None, None));

    assert!(!plain_secret.convert_to_latest(666));
    assert!(matches!(plain_secret, PlainSecret::Rev2(data) if data == rev2(None, None)));
}
