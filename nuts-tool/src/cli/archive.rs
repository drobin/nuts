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
pub mod migrate;

use anyhow::Result;
use clap::{ArgAction, Args, Subcommand};

use crate::cli::archive::add::ArchiveAddArgs;
use crate::cli::archive::create::ArchiveCreateArgs;
use crate::cli::archive::get::ArchiveGetArgs;
use crate::cli::archive::info::ArchiveInfoArgs;
use crate::cli::archive::list::ArchiveListArgs;
use crate::cli::archive::migrate::ArchiveMigrateArgs;
use crate::cli::ctx::{ArchiveContext, ContainerContext, GlobalContext};
use crate::cli::GlobalContainerArgs;

#[derive(Args, Clone, Debug)]
pub struct GlobalArchiveArgs {
    /// Starts the migration when the container/archive is opened
    #[clap(long, action = ArgAction::SetTrue, global = true)]
    pub migrate: bool,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct ArchiveArgs {
    #[clap(subcommand)]
    command: Option<ArchiveCommand>,

    #[command(flatten)]
    global_args: GlobalContainerArgs,

    #[command(flatten)]
    global_archive_args: GlobalArchiveArgs,
}

impl ArchiveArgs {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        let ctx = ContainerContext::new(ctx, &self.global_args);
        let ctx = ArchiveContext::new(&ctx, &self.global_archive_args);

        self.command
            .as_ref()
            .map_or(Ok(()), |command| command.run(&ctx))
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

    /// Performs migration tasks
    Migrate(ArchiveMigrateArgs),
}

impl ArchiveCommand {
    pub fn run(&self, ctx: &ArchiveContext) -> Result<()> {
        match self {
            Self::Add(args) => args.run(ctx),
            Self::Create(args) => args.run(ctx),
            Self::Get(args) => args.run(ctx),
            Self::Info(args) => args.run(ctx),
            Self::List(args) => args.run(ctx),
            Self::Migrate(args) => args.run(ctx),
        }
    }
}
