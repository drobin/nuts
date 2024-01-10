// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

use crate::digest::Digest;
use crate::kdf::Kdf;
use crate::tests::RND;

#[test]
fn ok() {
    match Kdf::pbkdf2(Digest::Sha1, 5, &[1, 2, 3]) {
        Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        } => {
            assert_eq!(digest, Digest::Sha1);
            assert_eq!(iterations, 5);
            assert_eq!(salt, [1, 2, 3]);
        }
        _ => panic!("invalid kdf"),
    }
}

#[test]
fn generate_empty_salt() {
    let kdf = Kdf::generate_pbkdf2(Digest::Sha1, 5, 0).unwrap();

    match kdf {
        Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        } => {
            assert_eq!(digest, Digest::Sha1);
            assert_eq!(iterations, 5);
            assert_eq!(salt, [0; 0]);
        }
        _ => panic!("invalid kdf"),
    }
}

#[test]
fn generate_with_salt() {
    let kdf = Kdf::generate_pbkdf2(Digest::Sha1, 5, 3).unwrap();

    match kdf {
        Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        } => {
            assert_eq!(digest, Digest::Sha1);
            assert_eq!(iterations, 5);
            assert_eq!(salt.len(), 3); // salt filled with random data
            assert_eq!(salt, &RND[..3]);
        }
        _ => panic!("invalid kdf"),
    }
}

#[test]
#[should_panic(expected = "invalid password, cannot be empty")]
fn create_key_empty_password() {
    Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![1, 2, 3],
    }
    .create_key(b"")
    .unwrap();
}

#[test]
#[should_panic(expected = "invalid salt, cannot be empty")]
fn create_key_empty_salt() {
    Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![],
    }
    .create_key(b"123")
    .unwrap();
}

#[test]
fn create_key() {
    let wkey = Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![1, 2, 3],
    }
    .create_key(b"123")
    .unwrap();

    assert_eq!(
        *wkey,
        vec![
            96, 23, 159, 91, 244, 187, 88, 88, 95, 129, 91, 252, 136, 14, 242, 207, 92, 3, 153, 56
        ]
    );
}
