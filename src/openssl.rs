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

#[cfg(test)]
mod tests;

use ::openssl as ossl;

use crate::error::Error;
use crate::result::Result;
use crate::types::Digest;

#[cfg(not(test))]
pub fn random(target: &mut [u8]) -> Result<()> {
    ossl::rand::rand_bytes(target).map(|_| ()).or_else(|err| {
        let msg = format!("{}", err);
        Err(Error::Rand(msg))
    })
}

#[cfg(test)]
pub const RND: [u8; 32] = [
    33, 155, 95, 60, 65, 96, 253, 183, 93, 150, 39, 110, 253, 132, 24, 187, 194, 29, 136, 183, 170,
    65, 174, 63, 126, 229, 61, 66, 15, 128, 146, 43,
];

#[cfg(test)]
pub fn random(target: &mut [u8]) -> Result<()> {
    assert!(target.len() <= RND.len());
    target.clone_from_slice(&RND[..target.len()]);
    Ok(())
}

pub enum HMAC {}

impl HMAC {
    pub fn create(digest: Digest, key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let md = digest_to_openssl(digest);
        let key = ossl::pkey::PKey::hmac(key).or_else(HMAC::as_error)?;
        let mut signer = ossl::sign::Signer::new(md, &key).or_else(HMAC::as_error)?;

        let hmac = signer.sign_oneshot_to_vec(data).or_else(HMAC::as_error)?;
        Ok(hmac)
    }

    pub fn verify(digest: Digest, key: &[u8], data: &[u8], hmac: &[u8]) -> Result<()> {
        let hmac2 = HMAC::create(digest, key, data)?;

        if ossl::memcmp::eq(hmac, &hmac2) {
            Ok(())
        } else {
            Err(Error::HmacMismatch)
        }
    }

    fn as_error<T>(stack: ossl::error::ErrorStack) -> Result<T> {
        let msg = format!("{}", stack);
        Err(Error::Hmac(msg))
    }
}

fn digest_to_openssl(digest: Digest) -> ossl::hash::MessageDigest {
    match digest {
        Digest::Sha1 => ::openssl::hash::MessageDigest::sha1(),
    }
}
