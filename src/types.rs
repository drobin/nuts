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

use crate::error::Error;
use crate::result::Result;

/// Supported cipher algorithms.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cipher {
    /// No encryption
    None,

    /// AES with a 128-bit key in CTR mode
    Aes128Ctr,
}

impl Cipher {
    /// Returns the key size of the cipher.
    pub fn key_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 16,
        }
    }

    /// Returns the IV size of the cipher.
    pub fn iv_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 16,
        }
    }

    /// Returns the block size of the cipher.
    pub fn block_size(&self) -> u32 {
        match self {
            Cipher::None => 1,
            Cipher::Aes128Ctr => 1,
        }
    }
}

/// Supported message digests.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Digest {
    /// SHA1
    Sha1,
}

impl Digest {
    /// Return the size of the message digest.
    ///
    /// This is the size of the resulting hash.
    pub fn size(&self) -> u32 {
        match self {
            Digest::Sha1 => 20,
        }
    }
}

/// Supported wrapping key algorithms.
///
/// Based on a password provided by the user one of the algorithms are used to
/// calculate a wrapping key. The wrapping key then is used for encryption of
/// the secret in the header of the container.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum WrappingKey {
    /// PBKDF2
    Pbkdf2 {
        /// Number of iterations used by PBKDF2.
        iterations: u32,

        /// Length of salt value generated for PBKDF2.
        salt_len: u32,
    },
}

/// Container disk types.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DiskType {
    /// Space for the container is allocated once during creation of the
    /// container, unused blocks are initialized with all zeros.
    FatZero,

    /// Space for the container is allocated once during creation of the
    /// container, unused blocks are initialized with random data.
    FatRandom,

    /// Space for the container is allocated dynamically when needed, unused
    /// blocks are initialized with all zeros.
    ThinZero,

    /// Space for the container is allocated dynamically when needed, unused
    /// blocks are initialized with random data.
    ThinRandom,
}

/// The minimum size of a block.
pub const BLOCK_MIN_SIZE: u32 = 512;

/// Options to customize the creation of a container.
///
/// Use [`default()`] to create a set of default parameters. The
/// [`default_with_sizes()`] functions creates a set to default parameters with
/// some custom sizes.
///
/// [`default()`]: #method.default
/// [`default_with_sizes()`]: #method.default_with_sizes
pub struct Options {
    /// The disk type.
    pub dtype: DiskType,

    /// The wrapping key algorithm.
    pub wkey: WrappingKey,

    /// Cipher used by the container.
    pub cipher: Cipher,

    /// Message digest used by the container.
    pub md: Digest,

    /// The size of a single block.
    bsize: u32,

    /// Number of blocks.
    blocks: u64,
}

impl Options {
    /// Creates a set of defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default();
    ///
    /// assert_eq!(options.dtype, DiskType::FatRandom);
    /// assert_eq!(
    ///     options.wkey,
    ///     WrappingKey::Pbkdf2 {
    ///         iterations: 65536,
    ///         salt_len: 16
    ///     }
    /// );
    /// assert_eq!(options.cipher, Cipher::Aes128Ctr);
    /// assert_eq!(options.md, Digest::Sha1);
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    pub fn default() -> Options {
        Options {
            dtype: DiskType::FatRandom,
            wkey: WrappingKey::Pbkdf2 {
                iterations: 65536,
                salt_len: 16,
            },
            cipher: Cipher::Aes128Ctr,
            md: Digest::Sha1,
            bsize: BLOCK_MIN_SIZE,
            blocks: (1024 * 1024 / BLOCK_MIN_SIZE) as u64, // container of 1MB
        }
    }

    /// Creates a set of defaults with some custom sizes.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `bsize` or
    /// `blocks` are invalid.
    ///
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default_with_sizes(1024, 2).unwrap();
    ///
    /// assert_eq!(options.dtype, DiskType::FatRandom);
    /// assert_eq!(
    ///     options.wkey,
    ///     WrappingKey::Pbkdf2 {
    ///         iterations: 65536,
    ///         salt_len: 16
    ///     }
    /// );
    /// assert_eq!(options.cipher, Cipher::Aes128Ctr);
    /// assert_eq!(options.md, Digest::Sha1);
    /// assert_eq!(options.bsize(), 1024);
    /// assert_eq!(options.blocks(), 2);
    pub fn default_with_sizes(bsize: u32, blocks: u64) -> Result<Options> {
        let mut options = Options::default();
        options.update_sizes(bsize, blocks)?;
        Ok(options)
    }

    /// Returns the block size.
    pub fn bsize(&self) -> u32 {
        self.bsize
    }

    /// Returns the number of blocks.
    pub fn blocks(&self) -> u64 {
        self.blocks
    }

    /// Updates both size attributes of the options.
    ///
    /// The `bsize` argument is the block size and must be a multiple of
    /// [`BLOCK_MIN_SIZE`] bytes. You cannot have a block size less that
    /// [`BLOCK_MIN_SIZE`] bytes!
    ///
    /// The `blocks` argument specifies the number of blocks, which should be
    /// allocated for the container. It must be a number greater than `0`.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `bsize` or
    /// `blocks` are invalid.
    ///
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    /// [`BLOCK_MIN_SIZE`]: constant.BLOCK_MIN_SIZE.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::Options;
    ///
    /// let mut options = Options::default();
    ///
    /// assert!(options.update_sizes(1024, 2).is_ok());
    /// assert_eq!(options.bsize(), 1024);
    /// assert_eq!(options.blocks(), 2);
    /// ```
    pub fn update_sizes(&mut self, bsize: u32, blocks: u64) -> Result<()> {
        if bsize < BLOCK_MIN_SIZE {
            let message = format!(
                "Invalid block size, got {} but must be at least {}.",
                bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        if bsize % BLOCK_MIN_SIZE != 0 {
            let message = format!(
                "Invalid block size, got {} but must be a multiple of {}.",
                bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        if blocks == 0 {
            let message = format!("Invalid number of blocks, got {}, expected > 0.", blocks);
            return Err(Error::InvalArg(message));
        }

        self.bsize = bsize;
        self.blocks = blocks;

        Ok(())
    }
}
