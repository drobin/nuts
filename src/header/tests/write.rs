// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use crate::header::Header;
use crate::types::{Cipher, Digest, WrappingKey};
use crate::wkey::WrappingKeyData;

fn ok_header() -> Header {
    Header {
        revision: 1,
        cipher: Cipher::Aes128Ctr,
        digest: Some(Digest::Sha1),
        wrapping_key: Some(WrappingKeyData {
            wkey: WrappingKey::Pbkdf2 {
                iterations: 4711,
                salt_len: 3,
            },
            pbkdf2: Some(vec![1, 2, 3]),
        }),
        hmac: vec![4, 5, 6, 7],
        secret: vec![8, 9, 10, 11, 12],
    }
}

#[test]
fn ok() {
    let header = ok_header();
    let mut target = [0; 39];

    assert_eq!(header.write(&mut target).unwrap(), 39);
    assert_eq!(target[0..7], [b'n', b'u', b't', b's', b'-', b'i', b'o']); // magic
    assert_eq!(target[7], 1); // revision
    assert_eq!(target[8], 1); // cipher
    assert_eq!(target[9], 1); // digest
    assert_eq!(
        target[10..22],
        [1, 0x00, 0x00, 0x12, 0x67, 0x00, 0x00, 0x00, 0x03, 1, 2, 3]
    ); // pbkdf2
    assert_eq!(target[22..30], [0x00, 0x00, 0x00, 0x04, 4, 5, 6, 7]); // hmac
    assert_eq!(target[30..], [0x00, 0x00, 0x00, 0x05, 8, 9, 10, 11, 12]); // secret
}

#[test]
fn no_space() {
    let header = ok_header();
    let mut target = [0; 39];

    for i in 1..39 {
        let err = format!(
            "{:?}",
            header.write(&mut target.get_mut(..i).unwrap()).unwrap_err()
        );
        assert_eq!(err, "NoSpace");
    }
}
