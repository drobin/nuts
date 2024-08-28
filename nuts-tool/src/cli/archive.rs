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

pub mod add;
pub mod create;
pub mod get;
pub mod info;
pub mod list;

use anyhow::Result;
use clap::{Args, Subcommand};
use nuts_archive::{Archive, ArchiveFactory};
use nuts_container::Container;

use crate::backend::PluginBackend;
use crate::cli::archive::add::ArchiveAddArgs;
use crate::cli::archive::create::ArchiveCreateArgs;
use crate::cli::archive::get::ArchiveGetArgs;
use crate::cli::archive::info::ArchiveInfoArgs;
use crate::cli::archive::list::ArchiveListArgs;
use crate::cli::open_builder;

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct ArchiveArgs {
    #[clap(subcommand)]
    command: Option<ArchiveCommand>,
}

impl ArchiveArgs {
    pub fn run(&self) -> Result<()> {
        self.command
            .as_ref()
            .map_or(Ok(()), |command| command.run())
    }
}

#[derive(Debug, Subcommand)]
pub enum ArchiveCommand {
    /// Adds a new entry at the end of the archive
    Add(ArchiveAddArgs),

    /// Creates a new archive
    Create(ArchiveCreateArgs),

    /// Retrieve the content of an entry
    Get(ArchiveGetArgs),

    /// Prints information about the archive
    Info(ArchiveInfoArgs),

    /// Lists the content of the archive
    List(ArchiveListArgs),
}

impl ArchiveCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Add(args) => args.run(),
            Self::Create(args) => args.run(),
            Self::Get(args) => args.run(),
            Self::Info(args) => args.run(),
            Self::List(args) => args.run(),
        }
    }
}

fn open_archive(name: &str, verbose: u8) -> Result<Archive<PluginBackend>> {
    let (plugin_builder, options) = open_builder(name, verbose)?;

    Container::open_service::<_, ArchiveFactory>(plugin_builder, options).map_err(|err| err.into())
}
