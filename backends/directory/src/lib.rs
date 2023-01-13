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
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{cmp, result};

use nuts_backend::{Backend, BLOCK_MIN_SIZE};

pub use error::{DirectoryError, DirectoryResult};
pub use id::DirectoryId;
pub use info::DirectoryInfo;
pub use options::{DirectoryCreateOptions, DirectoryOpenOptions, DirectorySettings};

#[derive(Debug)]
pub struct DirectoryBackend {
    bsize: u32,
    path: PathBuf,
}

impl DirectoryBackend {
    fn open_read(&self, id: &DirectoryId) -> io::Result<File> {
        let path = id.to_pathbuf(&self.path);
        OpenOptions::new().read(true).open(path)
    }

    fn open_write(&self, id: &DirectoryId, aquire: bool) -> io::Result<File> {
        let path = id.to_pathbuf(&self.path);

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let fh = if aquire {
            OpenOptions::new().write(true).create_new(true).open(path)?
        } else {
            OpenOptions::new().write(true).create(true).open(path)?
        };

        Ok(fh)
    }
}

impl Backend for DirectoryBackend {
    type CreateOptions = DirectoryCreateOptions;
    type OpenOptions = DirectoryOpenOptions;
    type Settings = DirectorySettings;
    type Err = DirectoryError;
    type Id = DirectoryId;
    type Info = DirectoryInfo;

    fn create(options: DirectoryCreateOptions) -> DirectoryResult<(Self, DirectorySettings)> {
        let path = options.path.clone();

        if !options.overwrite {
            let header_path = DirectoryId::header().to_pathbuf(&path);

            if header_path.exists() {
                return Err(DirectoryError::Exists);
            }
        }

        let backend = DirectoryBackend {
            bsize: options.bsize,
            path,
        };
        let envelope = options.into();

        Ok((backend, envelope))
    }

    fn open(options: DirectoryOpenOptions) -> DirectoryResult<Self> {
        Ok(DirectoryBackend {
            bsize: BLOCK_MIN_SIZE,
            path: options.path,
        })
    }

    fn open_ready(&mut self, envelope: Self::Settings) {
        self.bsize = envelope.bsize;
    }

    fn info(&self) -> DirectoryResult<DirectoryInfo> {
        Ok(DirectoryInfo { bsize: self.bsize })
    }

    fn block_size(&self) -> u32 {
        self.bsize
    }

    fn header_id(&self) -> DirectoryId {
        DirectoryId::header()
    }

    fn aquire(&mut self) -> DirectoryResult<Self::Id> {
        const MAX: u8 = 3;

        for n in 0..MAX {
            let id = DirectoryId::generate();

            match self.open_write(&id, true) {
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

        Err(DirectoryError::UniqueId)
    }

    fn release(&mut self, id: Self::Id) -> DirectoryResult<()> {
        let path = id.to_pathbuf(&self.path);

        Ok(fs::remove_file(&path)?)
    }

    fn read(&mut self, id: &DirectoryId, buf: &mut [u8]) -> result::Result<usize, Self::Err> {
        let len = cmp::min(buf.len(), self.bsize as usize);
        let target = &mut buf[..len];

        let mut fh = self.open_read(id)?;

        fh.read_exact(target)?;

        Ok(len)
    }

    fn write(&mut self, id: &DirectoryId, buf: &[u8]) -> result::Result<usize, Self::Err> {
        let len = cmp::min(buf.len(), self.bsize as usize);
        let pad_len = self.bsize as usize - len;

        let mut fh = self.open_write(id, false)?;

        fh.write_all(&buf[..len])?;
        fh.write_all(&vec![0; pad_len])?;
        fh.flush()?;

        Ok(len)
    }
}
