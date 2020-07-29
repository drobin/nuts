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

use log::error;
use openssl::pkcs5;
use std::fmt;

use crate::error::Error;
use crate::result::Result;
use crate::types::Digest;

#[derive(PartialEq)]
pub struct Pbkdf2Data {
    pub iterations: u32,
    pub salt: Vec<u8>,
}

#[derive(PartialEq)]
pub enum WrappingKeyData {
    Pbkdf2(Pbkdf2Data),
}

impl WrappingKeyData {
    pub fn pbkdf2(iterations: u32, salt: &Vec<u8>) -> WrappingKeyData {
        let value = Pbkdf2Data {
            iterations,
            salt: salt.to_vec(),
        };

        WrappingKeyData::Pbkdf2(value)
    }

    pub fn key(&self, password: &[u8], digest: Digest) -> Result<Vec<u8>> {
        if password.is_empty() {
            let msg = format!("invalid password, cannot be empty");
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let WrappingKeyData::Pbkdf2(value) = self;

        if value.salt.is_empty() {
            let msg = format!("invalid salt, cannot be empty");
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let hash = digest.to_openssl();
        let mut key = vec![0; digest.size() as usize];

        pkcs5::pbkdf2_hmac(
            password,
            &value.salt,
            value.iterations as usize,
            hash,
            &mut key,
        )?;

        Ok(key)
    }
}

impl fmt::Debug for WrappingKeyData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WrappingKeyData::Pbkdf2(value) => {
                let salt = format!("<{} bytes>", value.salt.len());
                fmt.debug_struct("Pbkdf2Data")
                    .field("iterations", &value.iterations)
                    .field("salt", &salt)
                    .finish()
            }
        }
    }
}
