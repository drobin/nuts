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

use nuts_backend::Binary;
use std::path::Path;

use crate::id::Id;

const ID: [u8; 16] = [
    0xdb, 0x3d, 0x05, 0x23, 0xd4, 0x50, 0x75, 0x30, 0xe8, 0x6d, 0xf9, 0x6a, 0x1b, 0x76, 0xaa, 0x0c,
];
const ID_SLICE: &str = "db3d0523d4507530e86df96a1b76aa0c";

#[test]
fn generate() {
    let id = Id::generate();
    assert_eq!(id.0, ID);
}

#[test]
fn min() {
    let id = Id::min();
    assert_eq!(id.0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn as_hex() {
    let id = Id::generate();
    assert_eq!(id.as_hex(), ID_SLICE);
}

#[test]
fn to_pathbuf() {
    let id = Id::generate();
    let path = id.to_pathbuf(Path::new("foo"));

    assert_eq!(
        format!("{}", path.display()),
        "foo/db/3d/0523d4507530e86df96a1b76aa0c"
    );
}

#[test]
fn from_str() {
    let id = ID_SLICE.parse::<Id>().unwrap();
    assert_eq!(id.0, ID);
}

#[test]
fn from_str_inval_len() {
    let err = ID_SLICE[..31].parse::<Id>().unwrap_err();

    assert_eq!(
        format!("{}", err),
        "The id 'db3d0523d4507530e86df96a1b76aa0' is invalid"
    );
}

#[test]
fn from_str_inval_char() {
    let mut inval_id = String::from("x");
    inval_id.push_str(&ID_SLICE[1..]);

    assert_eq!(inval_id.len(), 32);

    let err = inval_id.parse::<Id>().unwrap_err();

    assert_eq!(
        format!("{}", err),
        "The id 'xb3d0523d4507530e86df96a1b76aa0c' is invalid"
    );
}

#[test]
fn de() {
    let id = Id::from_bytes(ID.as_slice()).unwrap();
    assert_eq!(id.0, ID);
}

#[test]
fn ser() {
    let buf = Id::generate().as_bytes();

    assert_eq!(buf, ID);
}
