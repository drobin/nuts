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

mod error;
mod id;
mod info;
mod options;

use log::warn;
use std::cmp;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use nuts::backend::{Backend, HeaderGet, HeaderSet, HEADER_MAX_SIZE};

pub use error::Error;
pub use id::Id;
pub use info::Info;
pub use options::{CreateOptions, OpenOptions, Settings};

use crate::error::Result;

fn open_read(path: &Path, id: &Id) -> io::Result<File> {
    let path = id.to_pathbuf(path);
    fs::OpenOptions::new().read(true).open(path)
}

fn open_write(path: &Path, id: &Id, aquire: bool) -> io::Result<File> {
    let path = id.to_pathbuf(path);

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let fh = if aquire {
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?
    } else {
        fs::OpenOptions::new().write(true).create(true).open(path)?
    };

    Ok(fh)
}

fn read_block(path: &Path, id: &Id, bsize: u32, buf: &mut [u8]) -> Result<usize> {
    let len = cmp::min(buf.len(), bsize as usize);
    let target = &mut buf[..len];

    let mut fh = open_read(path, id)?;

    fh.read_exact(target)?;

    Ok(len)
}

fn write_block(path: &Path, id: &Id, bsize: u32, buf: &[u8]) -> Result<usize> {
    let len = cmp::min(buf.len(), bsize as usize);
    let pad_len = bsize as usize - len;

    let mut fh = open_write(path, id, false)?;

    fh.write_all(&buf[..len])?;
    fh.write_all(&vec![0; pad_len])?;
    fh.flush()?;

    Ok(len)
}

fn read_header(path: &Path, buf: &mut [u8]) -> Result<()> {
    read_block(path, &Id::min(), HEADER_MAX_SIZE as u32, buf).map(|_| ())
}

fn write_header(path: &Path, bsize: u32, buf: &[u8]) -> Result<()> {
    write_block(path, &Id::min(), bsize, buf).map(|_| ())
}

#[derive(Debug)]
pub struct DirectoryBackend {
    bsize: u32,
    path: PathBuf,
}

impl HeaderGet<Self> for DirectoryBackend {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<()> {
        read_header(&self.path, bytes)
    }
}

impl HeaderSet<Self> for DirectoryBackend {
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<()> {
        write_header(&self.path, self.bsize, bytes)
    }
}

impl Backend for DirectoryBackend {
    type CreateOptions = CreateOptions;
    type OpenOptions = OpenOptions;
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

    fn aquire(&mut self) -> Result<Self::Id> {
        const MAX: u8 = 3;

        for n in 0..MAX {
            let id = Id::generate();

            match open_write(&self.path, &id, true) {
                Ok(mut fh) => {
                    fh.flush()?;
                    return Ok(id);
                }
                Err(err) => {
                    if err.kind() == ErrorKind::AlreadyExists {
                        warn!("Id {} already exists try again ({}/{})", id, n + 1, MAX);
                    } else {
                        return Err(err.into());
                    }
                }
            }
        }

        Err(Error::UniqueId)
    }

    fn release(&mut self, id: Self::Id) -> Result<()> {
        let path = id.to_pathbuf(&self.path);

        Ok(fs::remove_file(&path)?)
    }

    fn read(&mut self, id: &Id, buf: &mut [u8]) -> Result<usize> {
        read_block(&self.path, id, self.bsize, buf)
    }

    fn write(&mut self, id: &Id, buf: &[u8]) -> Result<usize> {
        write_block(&self.path, id, self.bsize, buf)
    }
}
