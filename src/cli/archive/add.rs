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

use anyhow::{anyhow, Result};
use clap::{ArgGroup, Args};
use log::debug;
use nuts_archive::Archive;
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::cli::open_container;

#[derive(Args, Debug)]
#[clap(group(ArgGroup::new("input").required(true).multiple(false)))]
pub struct ArchiveAddArgs {
    /// The name of the entry.
    ///
    /// If specified reads the content from stdin.
    /// If not specified, the --path option is required.
    #[clap(group = "input")]
    name: Option<String>,

    /// File to be appended to the archive
    #[clap(short, long, group = "input")]
    path: Option<PathBuf>,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ArchiveAddArgs {
    fn entry_name<'a>(&'a self) -> Result<Cow<'a, str>> {
        if let Some(name) = self.name.as_deref() {
            Ok(Cow::Borrowed(name))
        } else if let Some(path) = self.path.as_deref() {
            if path.is_file() {
                Ok(path.to_string_lossy())
            } else {
                Err(anyhow!("Not a regular file: {}", path.display()))
            }
        } else {
            panic!("should never be reached")
        }
    }

    fn open_stream(&self) -> Result<Box<dyn Read>> {
        if self.name.is_some() {
            Ok(Box::new(io::stdin()))
        } else if let Some(path) = self.path.as_deref() {
            Ok(Box::new(File::open(path)?))
        } else {
            panic!("should never be reached")
        }
    }

    pub fn run(&self) -> Result<()> {
        let container = open_container(&self.container)?;
        let mut archive = Archive::open(container)?;
        let block_size = archive.as_ref().block_size() as usize;

        let name = self.entry_name()?;

        debug!("block size: {}", block_size);
        debug!("entry name: {}", name);

        let mut s = self.open_stream()?;
        let mut entry = archive.append(name).build()?;
        let mut buf = vec![0; block_size];

        loop {
            let n = s.read(&mut buf)?;
            debug!("read from input stream: {}", n);

            if n > 0 {
                entry.write_all(&buf[..n])?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
