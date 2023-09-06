// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error, fmt};

use crate::container::digest::Digest;
use crate::container::openssl::{evp, rand, OpenSSLError};
use crate::svec::SecureVec;

use super::DigestError;

#[derive(Debug)]
pub enum KdfNoneError {
    InvalidNumberOfArguments(usize),
}

impl fmt::Display for KdfNoneError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidNumberOfArguments(num) => write!(
                fmt,
                "invalid number of arguments for the none-kdf, expected none but got {}",
                num
            ),
        }
    }
}

impl error::Error for KdfNoneError {}

#[derive(Debug)]
pub enum KdfPbkdf2Error {
    InvalidNumberOfArguments(usize),
    InvalidDigest(DigestError),
    InvalidIterations(ParseIntError),
    InvalidSaltLen(ParseIntError),
    OpenSSL(OpenSSLError),
}

impl fmt::Display for KdfPbkdf2Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidNumberOfArguments(num) => write!(
                fmt,
                "invalid number of arguments for PBKDF2, got {} but none or three are expected",
                num
            ),
            Self::InvalidDigest(cause) => fmt::Display::fmt(cause, fmt),
            Self::InvalidIterations(cause) => fmt::Display::fmt(cause, fmt),
            Self::InvalidSaltLen(cause) => fmt::Display::fmt(cause, fmt),
            Self::OpenSSL(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl error::Error for KdfPbkdf2Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidNumberOfArguments(_) => None,
            Self::InvalidDigest(cause) => Some(cause),
            Self::InvalidIterations(cause) => Some(cause),
            Self::InvalidSaltLen(cause) => Some(cause),
            Self::OpenSSL(cause) => Some(cause),
        }
    }
}

impl From<DigestError> for KdfPbkdf2Error {
    fn from(cause: DigestError) -> Self {
        KdfPbkdf2Error::InvalidDigest(cause)
    }
}

impl From<OpenSSLError> for KdfPbkdf2Error {
    fn from(cause: OpenSSLError) -> Self {
        KdfPbkdf2Error::OpenSSL(cause)
    }
}

#[derive(Debug)]
pub enum KdfError {
    None(KdfNoneError),
    Pbkdf2(KdfPbkdf2Error),
    Unknown(String),
}

impl fmt::Display for KdfError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None(cause) => fmt::Display::fmt(cause, fmt),
            Self::Pbkdf2(cause) => fmt::Display::fmt(cause, fmt),
            Self::Unknown(str) => write!(fmt, "unknown kdf: {}", str),
        }
    }
}

impl error::Error for KdfError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::None(cause) => Some(cause),
            Self::Pbkdf2(cause) => Some(cause),
            Self::Unknown(_) => None,
        }
    }
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
#[derive(Clone, Deserialize, PartialEq, Serialize)]
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
    /// Creates a `Kdf` instance for the PBKDF2 algorithm.
    ///
    /// The `digest`, `iterations` and the `salt` values are used to customize
    /// the PBKDF2 algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts_container::container::*;
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
    /// use nuts_container::container::*;
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
    ) -> Result<Kdf, OpenSSLError> {
        let mut salt = vec![0; salt_len as usize];
        rand::rand_bytes(&mut salt)?;

        Ok(Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        })
    }

    pub(crate) fn create_key(&self, password: &[u8]) -> Result<SecureVec, OpenSSLError> {
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

                let md = digest.to_evp();
                let mut key = vec![0; digest.size()];

                evp::pbkdf2_hmac(password, salt, *iterations, md, &mut key)?;

                Ok(key.into())
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

fn parse_none(v: &[&str]) -> Result<Kdf, KdfNoneError> {
    if v.is_empty() {
        Ok(Kdf::None)
    } else {
        Err(KdfNoneError::InvalidNumberOfArguments(v.len()))
    }
}

fn parse_pbkdf2(v: &[&str]) -> Result<Kdf, KdfPbkdf2Error> {
    const DEFAULT_DIGEST: Digest = Digest::Sha1;
    const DEFAULT_ITERATIONS: u32 = 65536;
    const DEFAULT_SALT_LEN: u32 = 16;

    if v.len() != 0 && v.len() != 3 {
        return Err(KdfPbkdf2Error::InvalidNumberOfArguments(v.len()));
    }

    let digest = if v.is_empty() || v[0].is_empty() {
        DEFAULT_DIGEST
    } else {
        v[0].parse::<Digest>()?
    };

    let iterations = if v.is_empty() || v[1].is_empty() {
        DEFAULT_ITERATIONS
    } else {
        v[1].parse::<u32>()
            .map_err(|err| KdfPbkdf2Error::InvalidIterations(err))?
    };

    let salt_len = if v.is_empty() || v[2].is_empty() {
        DEFAULT_SALT_LEN
    } else {
        v[2].parse::<u32>()
            .map_err(|err| KdfPbkdf2Error::InvalidSaltLen(err))?
    };

    Ok(Kdf::generate_pbkdf2(digest, iterations, salt_len)?)
}

impl FromStr for Kdf {
    type Err = KdfError;

    fn from_str(s: &str) -> Result<Self, KdfError> {
        let v: Vec<&str> = s
            .split(':')
            .map(|s| s.trim_matches(char::is_whitespace))
            .collect();

        if v.is_empty() {
            todo!()
        }

        match v[0] {
            "none" => parse_none(&v[1..]).map_err(|err| KdfError::None(err)),
            "pbkdf2" => parse_pbkdf2(&v[1..]).map_err(|err| KdfError::Pbkdf2(err)),
            _ => Err(KdfError::Unknown(v[0].to_string())),
        }
    }
}
