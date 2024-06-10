// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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
use clap::{Args, Subcommand};
use log::debug;
use nuts_archive::Archive;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::archive::append_recursive;
use crate::cli::open_container;

#[derive(Args, Debug)]
// #[clap(group(ArgGroup::new("input").required(true).multiple(false)))]
#[clap(args_conflicts_with_subcommands = true)]
pub struct ArchiveAddArgs {
    #[clap(subcommand)]
    command: Option<ArchiveAddCommand>,

    /// Path to files/directories to be added to the archive. If PATHS contains
    /// a directory all entries in the directory are also appended. If no PATHS
    /// are specified an empty archive is created.
    paths: Vec<PathBuf>,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveAddArgs {
    pub fn run(&self) -> Result<()> {
        if let Some(command) = self.command.as_ref() {
            return command.run();
        }

        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let mut archive = Archive::open(container)?;

        for path in self.paths.iter() {
            append_recursive(&mut archive, path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum ArchiveAddCommand {
    /// Appends a custom file to the archive.
    File(ArchiveAddFileArgs),

    /// Appends a custom directory to the archive.
    Directory(ArchiveAddDirectoryArgs),

    /// Appends a custom symlink to the archive.
    Symlink(ArchiveAddSymlinkArgs),
}

impl ArchiveAddCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::File(args) => args.run(),
            Self::Directory(args) => args.run(),
            Self::Symlink(args) => args.run(),
        }
    }
}

#[derive(Args, Debug)]
pub struct ArchiveAddFileArgs {
    /// Name of the file.
    name: String,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveAddFileArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let mut archive = Archive::open(container)?;

        let block_size = archive.as_ref().block_size() as usize;
        let mut entry = archive.append_file(&self.name).build()?;
        let mut buf = vec![0; block_size];

        loop {
            let n = io::stdin().read(&mut buf)?;
            debug!("{} bytes read from stdin", n);

            if n > 0 {
                entry.write_all(&buf[..n])?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ArchiveAddDirectoryArgs {
    /// Name of the directory.
    name: String,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveAddDirectoryArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let mut archive = Archive::open(container)?;

        archive.append_directory(&self.name).build()?;

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ArchiveAddSymlinkArgs {
    /// Name of the symlink.
    name: String,

    /// Target of the symlink.
    target: String,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ArchiveAddSymlinkArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let mut archive = Archive::open(container)?;

        archive.append_symlink(&self.name, &self.target).build()?;

        Ok(())
    }
}
