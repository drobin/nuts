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
use log::{debug, error};

use crate::error::Error;
use crate::result::Result;
use crate::types::{Cipher, Digest};

#[cfg(not(test))]
pub fn random(target: &mut [u8]) -> Result<()> {
    ossl::rand::rand_bytes(target).map(|_| ())?;
    Ok(())
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
        let md = digest.to_openssl();
        let key = ossl::pkey::PKey::hmac(key)?;
        let mut signer = ossl::sign::Signer::new(md, &key)?;

        let hmac = signer.sign_oneshot_to_vec(data)?;
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
}

pub fn cipher(
    cipher: Cipher,
    encrypt: bool,
    input: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>> {
    let mut output = Vec::with_capacity(input.len());

    if let Some(ossl_cipher) = cipher.to_openssl() {
        if input.len() % ossl_cipher.block_size() != 0 {
            let msg = format!(
                "length of input {} mut be a multiple of block-size {}",
                input.len(),
                ossl_cipher.block_size()
            );
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let key = key.get(..ossl_cipher.key_len()).ok_or_else(|| {
            let msg = format!(
                "key too short, at least {} bytes needed but got {}",
                ossl_cipher.key_len(),
                key.len()
            );
            error!("{}", msg);
            Error::InvalArg(msg)
        })?;

        let iv = if let Some(len) = ossl_cipher.iv_len() {
            iv.get(..len).ok_or_else(|| {
                let msg = format!(
                    "iv too short, at least {} bytes needed but got {}",
                    len,
                    iv.len()
                );
                error!("{}", msg);
                Error::InvalArg(msg)
            })?
        } else {
            panic!("no support for a cipher without iv");
        };

        let mode = if encrypt {
            ossl::symm::Mode::Encrypt
        } else {
            ossl::symm::Mode::Decrypt
        };

        output.resize(input.len(), 0);

        let mut encrypter = ossl::symm::Crypter::new(ossl_cipher, mode, key, Some(iv))?;
        encrypter.pad(false);

        let count = encrypter.update(input, &mut output)?;
        assert_eq!(count, output.len());
    } else {
        assert_eq!(cipher, Cipher::None);
        output.extend(input);
    };

    if encrypt {
        debug!("{} bytes encrypted, cipher: {}", output.len(), cipher);
    } else {
        debug!("{} bytes decrypted, cipher: {}", output.len(), cipher);
    }

    Ok(output)
}
