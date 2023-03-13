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

use std::io::Cursor;

use nuts_bytes::{FromBytesExt, ToBytesExt};

use crate::id::DirectoryId;

const ID: [u8; 16] = [
    0xdb, 0x3d, 0x05, 0x23, 0xd4, 0x50, 0x75, 0x30, 0xe8, 0x6d, 0xf9, 0x6a, 0x1b, 0x76, 0xaa, 0x0c,
];
const ID_SLICE: &str = "db3d0523d4507530e86df96a1b76aa0c";

#[test]
fn generate() {
    let id = DirectoryId::generate();
    assert_eq!(id.0, ID);
}

#[test]
fn min() {
    let id = DirectoryId::min();
    assert_eq!(id.0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn max() {
    let id = DirectoryId::max();
    assert_eq!(
        id.0,
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff
        ]
    );
}

#[test]
fn as_hex() {
    let id = DirectoryId::generate();
    assert_eq!(id.as_hex(), ID_SLICE);
}

#[test]
fn to_pathbuf() {
    let id = DirectoryId::generate();
    let path = id.to_pathbuf("foo");

    assert_eq!(
        format!("{}", path.display()),
        "foo/db/3d/0523d4507530e86df96a1b76aa0c"
    );
}

#[test]
fn from_str() {
    let id = ID_SLICE.parse::<DirectoryId>().unwrap();
    assert_eq!(id.0, ID);
}

#[test]
fn from_str_inval_len() {
    let err = ID_SLICE[..31].parse::<DirectoryId>().unwrap_err();

    assert_eq!(
        format!("{}", err),
        "The id 'db3d0523d4507530e86df96a1b76aa0' is invalid"
    );
}

#[test]
fn from_str_inval_char() {
    let mut inval_id = String::from("x");
    inval_id.extend(ID_SLICE[1..].chars());

    assert_eq!(inval_id.len(), 32);

    let err = inval_id.parse::<DirectoryId>().unwrap_err();

    assert_eq!(
        format!("{}", err),
        "The id 'xb3d0523d4507530e86df96a1b76aa0c' is invalid"
    );
}

#[test]
fn from_bytes_eof() {
    let mut cursor = Cursor::new(&ID[..15]);
    let err = cursor.from_bytes::<DirectoryId>().unwrap_err();

    assert_eq!(format!("{:?}", err), "Eof");
}
#[test]
fn from_bytes() {
    let mut cursor = Cursor::new(ID);
    let id = cursor.from_bytes::<DirectoryId>().unwrap();

    assert_eq!(id.0, ID);
}

#[test]
fn to_bytes_nospace() {
    let mut buf = [0; 15];
    let mut cursor = Cursor::new(&mut buf[..]);

    let err = cursor.to_bytes(&DirectoryId::generate()).unwrap_err();
    assert_eq!(format!("{:?}", err), "NoSpace");
}

#[test]
fn to_bytes() {
    let mut cursor = Cursor::new(vec![]);

    cursor.to_bytes(&DirectoryId::generate()).unwrap();
    assert_eq!(cursor.into_inner(), ID);
}