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
use serde::Serialize;

use crate::container::digest::Digest;
use crate::container::kdf::Kdf;

#[test]
fn de_none() {
    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x00, // none variant
        ]
        .as_slice(),
    );
    let kdf = reader.deserialize::<Kdf>().unwrap();

    assert_eq!(kdf, Kdf::None);
}

#[test]
fn de_pbkdf2() {
    let mut reader = Reader::new(
        [
            0x00, 0x00, 0x00, 0x01, // pbkdf2 variant
            0x00, 0x00, 0x00, 0x00, // sha1
            0x00, 0x01, 0x00, 0x00, // iterations
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3, // salt
        ]
        .as_slice(),
    );
    let kdf = reader.deserialize::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3]
        }
    );
}

#[test]
fn ser_none() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(Kdf::None.serialize(&mut writer).unwrap(), 4);

    assert_eq!(
        writer.into_target(),
        [
            0x00,0x00,0x00,0x00, // none variant
        ]
    );
}

#[test]
fn ser_pbkdf2() {
    let mut writer = Writer::new(vec![]);
    assert_eq!(
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: vec![1, 2, 3],
        }
        .serialize(&mut writer)
        .unwrap(),
        23
    );

    assert_eq!(
        writer.into_target(),
        [
            0x00, 0x00, 0x00, 0x01, // pbkdf2 variant
            0x00, 0x00, 0x00, 0x00, // sha1
            0x000, 0x01, 0x00, 0x00, // iterations
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 1, 2, 3 // salt
        ]
    );
}
