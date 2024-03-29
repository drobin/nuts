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

pub mod archive;
pub mod container;

use anyhow::{anyhow, Result};
use clap::{crate_version, ArgAction, Parser, Subcommand};
use env_logger::Builder;
use log::debug;
use log::LevelFilter;
use nuts_container::{Container, OpenOptionsBuilder};
use nuts_directory::{DirectoryBackend, OpenOptions};
use rpassword::prompt_password;
use std::fs;
use std::path::PathBuf;

use crate::cli::archive::ArchiveArgs;
use crate::cli::container::ContainerArgs;

#[derive(Debug, Parser)]
#[clap(name = "nuts", bin_name = "nuts")]
#[clap(version = crate_version!())]
pub struct NutsCli {
    #[clap(subcommand)]
    command: Commands,

    /// Enable verbose output. Can be called multiple times
    #[clap(short, long, action = ArgAction::Count, global = true)]
    verbose: u8,

    /// Be quiet. Don't produce any output
    #[clap(short, long, action = ArgAction::SetTrue, global = true)]
    pub quiet: bool,
}

impl NutsCli {
    pub fn configure_logging(&self) {
        let filter = match self.verbose {
            0 => LevelFilter::Off,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Builder::new().filter_level(filter).init();
    }

    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// General container tasks
    Container(ContainerArgs),

    /// An archive on top of the container
    Archive(ArchiveArgs),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Container(args) => args.run(),
            Self::Archive(args) => args.run(),
        }
    }
}

fn tool_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => {
            let tool_dir = dir.join(".nuts");

            debug!("tool_dir: {}", tool_dir.display());

            if !tool_dir.is_dir() {
                debug!("creating tool dir {}", tool_dir.display());
                fs::create_dir(&tool_dir)?;
            }

            Ok(tool_dir)
        }
        None => Err(anyhow!("unable to locate home-directory")),
    }
}

fn open_container(name: &str) -> Result<Container<DirectoryBackend<PathBuf>>> {
    let path = container_dir_for(name)?;

    let builder = OpenOptionsBuilder::new().with_password_callback(ask_for_password);
    let options = builder.build::<DirectoryBackend<PathBuf>>()?;

    Ok(Container::open(OpenOptions::for_path(path), options)?)
}

fn container_dir() -> Result<PathBuf> {
    let parent = tool_dir()?;
    let dir = parent.join("container.d");

    debug!("container_dir: {}", dir.display());

    if !dir.is_dir() {
        debug!("creating container dir {}", dir.display());
        fs::create_dir(&dir)?;
    }

    Ok(dir)
}

fn container_dir_for<S: AsRef<str>>(name: S) -> Result<PathBuf> {
    let parent = container_dir()?;
    let dir = parent.join(name.as_ref());

    debug!("container_dir for {}: {}", name.as_ref(), dir.display());

    Ok(dir)
}

pub fn ask_for_password() -> Result<Vec<u8>, String> {
    let password = prompt_password("Enter a password: ").map_err(|err| err.to_string())?;
    Ok(password.as_bytes().to_vec())
}
