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

use nuts_container::memory::MemoryBackend;

use crate::error::Error;
use crate::header::{Header, Magic};
use crate::tests::{into_error, setup_container};
use crate::userdata::Userdata;

#[test]
fn load_or_create_blank() {
    let mut container = setup_container();
    let userdata = Userdata::create(&mut container).unwrap();
    let header = Header::load_or_create(&mut container, &userdata.id).unwrap();

    assert_eq!(header.revision, 1);
    assert_eq!(header.first, None);
    assert_eq!(header.last, None);
}

#[test]
fn load_or_create_exists() {
    let mut container = setup_container();
    let userdata = Userdata::create(&mut container).unwrap();

    Header::<MemoryBackend> {
        magic: Magic::new(),
        revision: 7,
        first: Some("8".parse().unwrap()),
        last: Some("9".parse().unwrap()),
    }
    .write(&mut container, &userdata.id)
    .unwrap();

    let header = Header::load_or_create(&mut container, &userdata.id).unwrap();

    assert_eq!(header.revision, 7);
    assert_eq!(header.first, Some("8".parse().unwrap()));
    assert_eq!(header.last, Some("9".parse().unwrap()));
}

#[test]
fn load_or_create_invalid() {
    let mut container = setup_container();
    let userdata = Userdata::create(&mut container).unwrap();

    Header::<MemoryBackend> {
        magic: Magic([b'x'; 12]),
        revision: 7,
        first: Some("8".parse().unwrap()),
        last: Some("9".parse().unwrap()),
    }
    .write(&mut container, &userdata.id)
    .unwrap();

    let err = Header::load_or_create(&mut container, &userdata.id).unwrap_err();
    let err = into_error!(err, Error::InvalidHeader);
    let msg = into_error!(err, nuts_bytes::Error::Serde);
    assert_eq!(msg, "invalid header-magic");
}
