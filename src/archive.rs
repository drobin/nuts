// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use anyhow::Result;
use colored::*;
use log::{debug, error, trace};
use nuts_archive::Archive;
use nuts_directory::DirectoryBackend;
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::{Path, PathBuf};

fn append_recursive_metadata(
    archive: &mut Archive<DirectoryBackend<PathBuf>>,
    path: &Path,
    metadata: &Metadata,
) -> Result<()> {
    let block_size = archive.as_ref().block_size() as usize;
    let mut entry = archive.append(path.to_string_lossy()).build()?;

    if metadata.is_file() {
        let mut fh = File::open(path)?;
        let mut buf = vec![0; block_size];

        loop {
            let n = fh.read(&mut buf)?;
            trace!("{} bytes read from {}", n, path.display());

            if n > 0 {
                entry.write_all(&buf[..n])?;
            } else {
                break;
            }
        }
    }

    println!("a {}", path.display());

    if path.is_dir() {
        for entry in path.read_dir()? {
            let child = entry?.path();

            append_recursive(archive, &child)?;
        }
    }

    Ok(())
}

pub fn append_recursive(
    archive: &mut Archive<DirectoryBackend<PathBuf>>,
    path: &Path,
) -> Result<()> {
    debug!("append {}", path.display());

    match path.metadata() {
        Ok(metadata) => append_recursive_metadata(archive, path, &metadata),
        Err(err) => {
            error!("{}", err);
            println!("{}", format!("! {}", path.display()).red());
            Ok(())
        }
    }
}
