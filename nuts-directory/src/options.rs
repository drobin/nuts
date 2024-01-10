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

use nuts_backend::{Create, HeaderGet, HeaderSet, Open, HEADER_MAX_SIZE};
use nuts_bytes::{FromBytes, ToBytes};
use std::path::Path;

use crate::error::{Error, Result};
use crate::id::Id;
use crate::{read_header, write_header, DirectoryBackend};

const BLOCK_MIN_SIZE: u32 = 512;

/// [Options](nuts_backend::Backend::CreateOptions) needed to create the
/// backend.
///
/// You must pass the path, where the directory tree should be stored, to
/// [`CreateOptions::for_path()`], if creating a `CreateOptions` instance.
///
/// Furthermore the following options can be specified:
///
/// * [`CreateOptions::with_overwrite()`]: If set to `true` an already existing
///   path is reused. **Note**: If you overwrite an existing path, the content
///   is not removed! If set to `false` and the base path exists, the build
///   operation aborts with [`Error::Exists`]. The default is `false`.
/// * [`CreateOptions::with_bsize()`]: Specifies the block size of the backend.
///   This is the number of bytes, which can  be stored in an individual block.
///   The minimum block size is 512 bytes. The default is `512`.
#[derive(Clone, Debug)]
pub struct CreateOptions<P: AsRef<Path>> {
    path: P,
    bsize: u32,
    overwrite: bool,
    header: Vec<u8>,
}

impl<P: AsRef<Path>> CreateOptions<P> {
    /// Creates a new `CreateOptions` instance.
    ///
    /// You must pass the `path`, where the directory tree should be stored, to
    /// the function.
    ///
    /// For further options default values are applied.
    pub fn for_path(path: P) -> Self {
        CreateOptions {
            path,
            bsize: BLOCK_MIN_SIZE,
            overwrite: false,
            header: vec![],
        }
    }

    /// Assigns a new overwrite flag to the options.
    ///
    /// If set to `true` an already existing path is reused. **Note**: If you
    /// overwrite an existing path, the content is not removed! If set to
    /// `false` and the base path exists, the build operation aborts with
    /// [`Error::Exists`].
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Assigns a new block size to the options.
    ///
    /// This is the number of bytes, which can  be stored in an individual
    /// block.
    pub fn with_bsize(mut self, bsize: u32) -> Self {
        self.bsize = bsize;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.bsize >= BLOCK_MIN_SIZE {
            Ok(())
        } else {
            Err(Error::InvalidBlockSize(self.bsize))
        }
    }
}

impl<P: AsRef<Path>> HeaderSet<DirectoryBackend<P>> for CreateOptions<P> {
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<()> {
        self.validate()?;

        // The header is written while the backend is created (in Create::build).
        // Here you simple cache the data for later usage.
        self.header.clear();
        self.header.extend_from_slice(bytes);

        Ok(())
    }
}

impl<P: AsRef<Path>> Create<DirectoryBackend<P>> for CreateOptions<P> {
    fn settings(&self) -> Settings {
        Settings { bsize: self.bsize }
    }

    fn build(self) -> Result<DirectoryBackend<P>> {
        self.validate()?;

        if !self.overwrite {
            let header_path = Id::min().to_pathbuf(self.path.as_ref());

            if header_path.exists() {
                return Err(Error::Exists);
            }
        }

        write_header(self.path.as_ref(), self.bsize, &self.header)?;

        Ok(DirectoryBackend {
            bsize: self.bsize,
            path: self.path,
        })
    }
}

/// [Options](nuts_backend::Backend::OpenOptions) needed to open the backend.
///
/// You must pass the path, where the directory tree is stored, to
/// [`OpenOptions::for_path()`], if creating a `OpenOptions` instance.
pub struct OpenOptions<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> OpenOptions<P> {
    /// Creates a new `OpenOptions` instance.
    ///
    /// You must pass the `path`, where the directory tree should is stored, to
    /// the function.
    pub fn for_path(path: P) -> OpenOptions<P> {
        OpenOptions { path }
    }
}

impl<P: AsRef<Path>> HeaderGet<DirectoryBackend<P>> for OpenOptions<P> {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<()> {
        read_header(self.path.as_ref(), bytes)
    }
}

impl<P: AsRef<Path>> Open<DirectoryBackend<P>> for OpenOptions<P> {
    fn build(self, settings: Settings) -> Result<DirectoryBackend<P>> {
        Ok(DirectoryBackend {
            bsize: settings.bsize,
            path: self.path,
        })
    }
}

/// [Settings](nuts_backend::Backend::Settings) used by the backend.
#[derive(Clone, Debug, FromBytes, ToBytes)]
pub struct Settings {
    bsize: u32,
}
