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

//! Nuts backend implementation where the blocks of the container are stored
//! in a file hierarchy.
//!
//! # Introduction
//!
//! The _nuts-directory_ crate implements a [nuts] backend where the blocks of
//! the container are stored in a file hierarchy. Each block is identified by
//! an [id](Id), which is basically a 16 byte random number.
//!
//! When storing a block to disks the path to the file is derived from the id:
//!
//! 1. The id is converted into a hex string.
//! 2. The path then would be:
//!    `<first two chars>/<next two chars>/<remaining chars>`
//!
//! The header of the container is stored in the file
//! `00/00/0000000000000000000000000000`.
//!
//! # Create a new backend instance
//!
//! The [`CreateOptions`] type is used to create a new backend instance, which
//! is passed to the [`Container::create`] method. You need at least a
//! directory where the backend put its blocks. See the [`CreateOptions`]
//! documentation for further options.
//!
//! # Open an existing backend
//!
//! The [`OpenOptions`] type is used to open a backend instance, which is
//! passed to the [`Container::open`] method. You need the directory where the
//! backend put its blocks.
//!
//! [nuts]: https://crates.io/crates/nuts-container
//! [`Container::create`]: https://docs.rs/nuts-container/latest/nuts_container/container/struct.Container.html#method.create
//! [`Container::open`]: https://docs.rs/nuts-container/latest/nuts_container/container/struct.Container.html#method.open

mod error;
mod id;
mod info;
mod options;

use log::warn;
use nuts_backend::{Backend, HeaderGet, HeaderSet, HEADER_MAX_SIZE};
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;
use std::{cmp, fs};

pub use error::Error;
pub use id::Id;
pub use info::Info;
pub use options::{CreateOptions, OpenOptions, Settings};

use crate::error::Result;

fn read_block(path: &Path, id: &Id, bsize: u32, buf: &mut [u8]) -> Result<usize> {
    let path = id.to_pathbuf(path);
    let mut fh = fs::OpenOptions::new().read(true).open(path)?;

    let len = cmp::min(buf.len(), bsize as usize);
    let target = &mut buf[..len];

    fh.read_exact(target)?;

    Ok(len)
}

fn write_block(
    path: &Path,
    id: &Id,
    aquire: bool,
    header: bool,
    bsize: u32,
    buf: &[u8],
) -> Result<usize> {
    let path = id.to_pathbuf(path);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    if aquire {
        // A block is aquired. Allow only to create non-existing files.
        if path.exists() {
            return Err(io::Error::new(
                ErrorKind::Other,
                format!("cannot aquire {}, already stored in {}", id, path.display()),
            )
            .into());
        }
    } else {
        // * The header block can be created even if it does not exist.
        // * Any other block must be aquired before, thus open should fail if the
        //   file does not exist.
        if !header && !path.is_file() {
            return Err(io::Error::new(
                ErrorKind::Other,
                format!("cannot open {}, no related file {}", id, path.display()),
            )
            .into());
        }
    }

    let tmp_path = path.with_extension("tmp");

    let mut fh = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp_path)?;

    let len = cmp::min(buf.len(), bsize as usize);
    let pad_len = bsize as usize - len;

    fh.write_all(&buf[..len])?;
    fh.write_all(&vec![0; pad_len])?;
    fh.flush()?;

    fs::rename(tmp_path, path)?;

    Ok(len)
}

fn read_header(path: &Path, buf: &mut [u8]) -> Result<()> {
    read_block(path, &Id::min(), HEADER_MAX_SIZE as u32, buf).map(|_| ())
}

fn write_header(path: &Path, bsize: u32, buf: &[u8]) -> Result<()> {
    write_block(path, &Id::min(), false, true, bsize, buf).map(|_| ())
}

#[derive(Debug)]
pub struct DirectoryBackend<P: AsRef<Path>> {
    bsize: u32,
    path: P,
}

impl<P: AsRef<Path>> HeaderGet<Self> for DirectoryBackend<P> {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<()> {
        read_header(self.path.as_ref(), bytes)
    }
}

impl<P: AsRef<Path>> HeaderSet<Self> for DirectoryBackend<P> {
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<()> {
        write_header(self.path.as_ref(), self.bsize, bytes)
    }
}

impl<P: AsRef<Path>> Backend for DirectoryBackend<P> {
    type CreateOptions = CreateOptions<P>;
    type OpenOptions = OpenOptions<P>;
    type Settings = Settings;
    type Err = Error;
    type Id = Id;
    type Info = Info;

    fn info(&self) -> Result<Info> {
        Ok(Info { bsize: self.bsize })
    }

    fn block_size(&self) -> u32 {
        self.bsize
    }

    fn aquire(&mut self, buf: &[u8]) -> Result<Self::Id> {
        const MAX: u8 = 3;

        for n in 0..MAX {
            let id = Id::generate();

            match write_block(self.path.as_ref(), &id, true, false, self.bsize, buf) {
                Ok(_) => return Ok(id),
                Err(Error::Io(err)) => {
                    if err.kind() == ErrorKind::AlreadyExists {
                        warn!("Id {} already exists try again ({}/{})", id, n + 1, MAX);
                    } else {
                        return Err(err.into());
                    }
                }
                Err(err) => return Err(err),
            };
        }

        Err(Error::UniqueId)
    }

    fn release(&mut self, id: Self::Id) -> Result<()> {
        let path = id.to_pathbuf(self.path.as_ref());

        Ok(fs::remove_file(&path)?)
    }

    fn read(&mut self, id: &Id, buf: &mut [u8]) -> Result<usize> {
        read_block(self.path.as_ref(), id, self.bsize, buf)
    }

    fn write(&mut self, id: &Id, buf: &[u8]) -> Result<usize> {
        write_block(self.path.as_ref(), id, false, false, self.bsize, buf)
    }
}
