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

#[cfg(test)]
mod tests;

use log::{debug, trace};
use openssl::error::ErrorStack;
use openssl::pkcs5::pbkdf2_hmac;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

use crate::buffer::{Buffer, BufferError, BufferMut};
use crate::digest::Digest;
use crate::ossl;
use crate::svec::SecureVec;

/// [`Kdf`] related error codes.
#[derive(Debug, Error)]
pub enum KdfError {
    /// An error in the OpenSSL library occured.
    #[error(transparent)]
    OpenSSL(#[from] ErrorStack),
}

/// Supported key derivation functions.
///
/// Defines data used to calculate a wrapping key.
///
/// The wrapping key is created used by an algorithm defined as a variant of
/// this enum. The variants holds fields to customize the algorithm.
///
/// Based on a password provided by the user one of the algorithms are used to
/// calculate a wrapping key. The wrapping key then is used for encryption of
/// the secret in the header of the container.
#[derive(Clone, PartialEq)]
pub enum Kdf {
    /// No key derivation
    None,

    /// PBKDF2
    Pbkdf2 {
        /// Digest used by PBKDF2.
        digest: Digest,

        /// Number of iterations used by PBKDF2.
        iterations: u32,

        /// A salt value used by PBKDF2.
        salt: Vec<u8>,
    },
}

impl Kdf {
    /// Tests whether this is a [`None`](Self::None) kdf.
    pub fn is_none(&self) -> bool {
        match self {
            Kdf::None => true,
            Kdf::Pbkdf2 {
                digest: _,
                iterations: _,
                salt: _,
            } => false,
        }
    }

    /// Tests whether this is a [`Pbkdf2`](Self::Pbkdf2) kdf.
    pub fn is_pbkdf2(&self) -> bool {
        match self {
            Kdf::None => false,
            Kdf::Pbkdf2 {
                digest: _,
                iterations: _,
                salt: _,
            } => true,
        }
    }

    /// Creates a `Kdf` instance for the PBKDF2 algorithm.
    ///
    /// The `digest`, `iterations` and the `salt` values are used to customize
    /// the PBKDF2 algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts_container::*;
    ///
    /// let pbkdf2 = Kdf::pbkdf2(Digest::Sha1, 5, &[1, 2, 3]);
    ///
    /// match pbkdf2 {
    ///     Kdf::Pbkdf2 {
    ///         digest,
    ///         iterations,
    ///         salt,
    ///     } => {
    ///         assert_eq!(digest, Digest::Sha1);
    ///         assert_eq!(iterations, 5);
    ///         assert_eq!(salt, [1, 2, 3]);
    ///     }
    ///     _ => panic!("invalid kdf"),
    /// }
    /// ```
    pub fn pbkdf2(digest: Digest, iterations: u32, salt: &[u8]) -> Kdf {
        Kdf::Pbkdf2 {
            digest,
            iterations,
            salt: salt.to_vec(),
        }
    }

    /// Generates a `Kdf` instance for the PBKDF2 algorithm.
    ///
    /// The `digest`and `iterations` value is used to customize the PBKDF2
    /// algorithm. For the [`salt`] `salt_len` bytes of random data are
    /// generated.
    ///
    /// # Errors
    ///
    /// This method will return an [`Error::OpenSSL`] error if there was an
    /// error generating the random data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts_container::*;
    ///
    /// let kdf = Kdf::generate_pbkdf2(Digest::Sha1, 5, 3).unwrap();
    ///
    /// match kdf {
    ///     Kdf::Pbkdf2 {
    ///         digest,
    ///         iterations,
    ///         salt,
    ///     } => {
    ///         assert_eq!(digest, Digest::Sha1);
    ///         assert_eq!(iterations, 5);
    ///         assert_eq!(salt.len(), 3); // salt filled with random data
    ///     }
    ///     _ => panic!("invalid kdf"),
    /// }
    /// ```
    ///
    /// [`salt`]: #variant.Pbkdf2.field.salt
    /// [`Error::OpenSSL`]: ../error/enum.Error.html#variant.OpenSSL
    pub fn generate_pbkdf2(
        digest: Digest,
        iterations: u32,
        salt_len: u32,
    ) -> Result<Kdf, KdfError> {
        let mut salt = vec![0; salt_len as usize];
        ossl::rand_bytes(&mut salt)?;

        Ok(Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        })
    }

    fn create_key_internal(&self, password: &[u8]) -> Result<SecureVec, KdfError> {
        match self {
            Kdf::None => Ok(vec![].into()),
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt,
            } => {
                if password.is_empty() {
                    panic!("invalid password, cannot be empty");
                }

                if salt.is_empty() {
                    panic!("invalid salt, cannot be empty");
                }

                let md = digest.as_openssl();
                let mut key = vec![0; digest.size()];

                pbkdf2_hmac(password, salt, *iterations as usize, md, &mut key)?;

                Ok(key.into())
            }
        }
    }

    pub(crate) fn create_key(
        &self,
        password: &[u8],
        min_len: usize,
    ) -> Result<SecureVec, KdfError> {
        let mut key = self.create_key_internal(password)?;

        // ignore min_len for None
        while !self.is_none() && key.len() < min_len {
            let xxx = self.create_key_internal(&key)?;
            key.extend(xxx.as_ref());

            trace!("create_key (step): len = {}", key.len());
        }

        debug!("create_key: min_len = {}, len = {}", min_len, key.len());

        Ok(key)
    }

    pub(crate) fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Kdf, BufferError> {
        let b = buf.get_u32()?;

        match b {
            0 => Ok(Kdf::None),
            1 => {
                let digest = Digest::get_from_buffer(buf)?;
                let iterations = buf.get_u32()?;
                let salt = buf.get_vec::<8>()?;

                Ok(Kdf::pbkdf2(digest, iterations, &salt))
            }
            _ => Err(BufferError::InvalidIndex("Kdf".to_string(), b)),
        }
    }

    pub(crate) fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        match self {
            Kdf::None => buf.put_u32(0),
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt,
            } => {
                buf.put_u32(1)?;
                digest.put_into_buffer(buf)?;
                buf.put_u32(*iterations)?;
                buf.put_vec::<8>(salt.as_slice())?;

                Ok(())
            }
        }
    }
}

impl fmt::Display for Kdf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kdf::None => fmt.write_str("none"),
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt,
            } => {
                write!(fmt, "pbkdf2:{}:{}:{}", digest, iterations, salt.len())
            }
        }
    }
}

impl fmt::Debug for Kdf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kdf::None => fmt.debug_struct("None").finish(),
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt,
            } => {
                let salt = format!("<{} bytes>", salt.len());
                fmt.debug_struct("Pbkdf2")
                    .field("digest", &digest)
                    .field("iterations", &iterations)
                    .field("salt", &salt)
                    .finish()
            }
        }
    }
}

fn parse_none(v: &[&str]) -> Result<Kdf, ParseKdfNoneError> {
    if v.is_empty() {
        Ok(Kdf::None)
    } else {
        Err(ParseKdfNoneError::InvalidNumberOfArguments(v.len()))
    }
}

fn parse_pbkdf2(v: &[&str]) -> Result<Kdf, ParseKdfPbkdf2Error> {
    const DEFAULT_DIGEST: Digest = Digest::Sha256;
    const DEFAULT_ITERATIONS: u32 = 65536;
    const DEFAULT_SALT_LEN: u32 = 16;

    if !v.is_empty() && v.len() != 3 {
        return Err(ParseKdfPbkdf2Error::InvalidNumberOfArguments(v.len()));
    }

    let digest = if v.is_empty() || v[0].is_empty() {
        DEFAULT_DIGEST
    } else {
        v[0].parse::<Digest>()
            .map_err(|()| ParseKdfPbkdf2Error::InvalidDigest(v[0].to_string()))?
    };

    let iterations = if v.is_empty() || v[1].is_empty() {
        DEFAULT_ITERATIONS
    } else {
        v[1].parse::<u32>()
            .map_err(ParseKdfPbkdf2Error::InvalidIterations)?
    };

    let salt_len = if v.is_empty() || v[2].is_empty() {
        DEFAULT_SALT_LEN
    } else {
        v[2].parse::<u32>()
            .map_err(ParseKdfPbkdf2Error::InvalidSaltLen)?
    };

    Ok(Kdf::generate_pbkdf2(digest, iterations, salt_len)?)
}

#[derive(Debug, Error)]
pub enum ParseKdfNoneError {
    #[error("invalid number of arguments for the none-kdf, expected none but got {0}")]
    InvalidNumberOfArguments(usize),
}

#[derive(Debug, Error)]
pub enum ParseKdfPbkdf2Error {
    #[error("invalid number of arguments for PBKDF2, got {0} but none or three are expected")]
    InvalidNumberOfArguments(usize),

    #[error("invalid digest: {0}")]
    InvalidDigest(String),

    #[error("invalid iterations: {0}")]
    InvalidIterations(#[source] ParseIntError),

    #[error("invalid salt length: {0}")]
    InvalidSaltLen(#[source] ParseIntError),

    #[error(transparent)]
    Kdf(#[from] KdfError),
}

#[derive(Debug, Error)]
pub enum ParseKdfError {
    #[error(transparent)]
    None(ParseKdfNoneError),

    #[error(transparent)]
    Pbkdf2(ParseKdfPbkdf2Error),

    #[error("unknown kdf: {0}")]
    Unknown(String),
}

impl FromStr for Kdf {
    type Err = ParseKdfError;

    fn from_str(s: &str) -> Result<Self, ParseKdfError> {
        let v: Vec<&str> = s
            .split(':')
            .map(|s| s.trim_matches(char::is_whitespace))
            .collect();

        if v.is_empty() {
            todo!()
        }

        match v[0] {
            "none" => parse_none(&v[1..]).map_err(ParseKdfError::None),
            "pbkdf2" => parse_pbkdf2(&v[1..]).map_err(ParseKdfError::Pbkdf2),
            _ => Err(ParseKdfError::Unknown(v[0].to_string())),
        }
    }
}
