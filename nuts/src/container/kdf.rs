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
use std::fmt;
use std::io::{Read, Write};

use nuts_backend::Backend;
use nuts_bytes::{FromBytes, FromBytesExt, ToBytes, ToBytesExt};

use crate::container::digest::Digest;
use crate::container::error::ContainerResult;
use crate::openssl::{evp, rand};
use crate::svec::SecureVec;

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
    /// use nuts::container::*;
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
    /// use nuts::container::*;
    /// use nutsbackend_directory::DirectoryBackend;
    ///
    /// let kdf = Kdf::generate_pbkdf2::<DirectoryBackend>(Digest::Sha1, 5, 3).unwrap();
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
    pub fn generate_pbkdf2<B: Backend>(
        digest: Digest,
        iterations: u32,
        salt_len: u32,
    ) -> ContainerResult<Kdf, B> {
        let mut salt = vec![0; salt_len as usize];
        rand::rand_bytes(&mut salt)?;

        Ok(Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        })
    }

    pub(crate) fn create_key<B: Backend>(&self, password: &[u8]) -> ContainerResult<SecureVec, B> {
        match self {
            Kdf::None => Ok(SecureVec::empty()),
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
                let mut key = SecureVec::zero(digest.size());

                evp::pbkdf2_hmac(password, salt, *iterations, md, &mut key)?;

                Ok(key)
            }
        }
    }
}

impl FromBytes for Kdf {
    fn from_bytes<R: Read>(source: &mut R) -> nuts_bytes::Result<Self> {
        let n = source.from_bytes()?;

        match n {
            0u8 => Ok(Kdf::None),
            1u8 => {
                let digest = source.from_bytes()?;
                let iterations = source.from_bytes()?;
                let salt = source.from_bytes::<Vec<u8>>()?;

                Ok(Kdf::pbkdf2(digest, iterations, &salt))
            }
            _ => Err(nuts_bytes::Error::invalid(format!("invalid kdf: {}", n))),
        }
    }
}

impl ToBytes for Kdf {
    fn to_bytes<W: Write>(&self, target: &mut W) -> nuts_bytes::Result<()> {
        match self {
            Kdf::None => {
                target.to_bytes(&0u8)?;
            }
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt,
            } => {
                target.to_bytes(&1u8)?;
                target.to_bytes(digest)?;
                target.to_bytes(iterations)?;
                target.to_bytes(&salt.as_slice())?;
            }
        }

        Ok(())
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
