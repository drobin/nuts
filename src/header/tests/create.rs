// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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
use crate::rand::RND;
use crate::types::{Cipher, Digest, Options, WrappingKey};

#[test]
fn cipher_none() {
    let options = Options::default_with_cipher(Cipher::None).unwrap();
    let header = Header::create(&options).unwrap();

    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::None);
    assert_eq!(header.digest, None);
    assert_eq!(header.wrapping_key, None);
    assert!(header.wrapping_iv.is_empty());
}

#[test]
fn cipher_aes128_ctr() {
    let options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();
    let header = Header::create(&options).unwrap();

    assert_eq!(header.revision, 1);
    assert_eq!(header.cipher, Cipher::Aes128Ctr);
    assert_eq!(header.digest, Some(Digest::Sha1));
    assert_eq!(
        header.wrapping_key,
        Some(WrappingKey::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: RND[..16].to_vec()
        })
    );
    assert_eq!(header.wrapping_iv, &RND[..16]);
}
