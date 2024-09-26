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

use crate::digest::Digest;
use crate::kdf::{Kdf, ParseKdfError, ParseKdfNoneError, ParseKdfPbkdf2Error};
use crate::tests::RND;

#[test]
fn from_str_invalid() {
    let err = "xxx".parse::<Kdf>().unwrap_err();
    assert!(matches!(err, ParseKdfError::Unknown(str) if str == "xxx"));
}

#[test]
fn from_str_none() {
    assert_eq!("none".parse::<Kdf>().unwrap(), Kdf::None);
}

#[test]
fn from_str_none_inval_args() {
    let err = "none:xxx".parse::<Kdf>().unwrap_err();
    assert!(matches!(err, ParseKdfError::None(cause)
            if matches!(cause, ParseKdfNoneError::InvalidNumberOfArguments(num)
                if num == 1)));
}

#[test]
fn from_str_pbkdf2_inval_args() {
    for (str, args) in [("pbkdf2::", 2), ("pbkdf2::::", 4)] {
        let err = str.parse::<Kdf>().unwrap_err();
        assert!(matches!(err, ParseKdfError::Pbkdf2(cause)
                if matches!(cause, ParseKdfPbkdf2Error::InvalidNumberOfArguments(num)
                    if num == args)));
    }
}

#[test]
fn from_str_pbkdf2_no_args() {
    let kdf = "pbkdf2".parse::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha256,
            iterations: 65536,
            salt: RND[..16].to_vec()
        }
    )
}

#[test]
fn from_str_pbkdf2_all_args() {
    let kdf = "pbkdf2:sha1:1:2".parse::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 1,
            salt: RND[..2].to_vec()
        }
    )
}

#[test]
fn from_str_pbkdf2_default_digest() {
    let kdf = "pbkdf2::1:2".parse::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha256,
            iterations: 1,
            salt: RND[..2].to_vec()
        }
    )
}

#[test]
fn from_str_pbkdf2_default_iterations() {
    let kdf = "pbkdf2:sha1::2".parse::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 65536,
            salt: RND[..2].to_vec()
        }
    )
}

#[test]
fn from_str_pbkdf2_default_salt_len() {
    let kdf = "pbkdf2:sha1:1:".parse::<Kdf>().unwrap();

    assert_eq!(
        kdf,
        Kdf::Pbkdf2 {
            digest: Digest::Sha1,
            iterations: 1,
            salt: RND[..16].to_vec()
        }
    )
}

#[test]
fn to_string_none() {
    assert_eq!(Kdf::None.to_string(), "none");
}

#[test]
fn to_string_pbkdf2() {
    let kdf = Kdf::Pbkdf2 {
        digest: Digest::Sha1,
        iterations: 1,
        salt: vec![1, 2, 3],
    };
    assert_eq!(kdf.to_string(), "pbkdf2:sha1:1:3");
}
