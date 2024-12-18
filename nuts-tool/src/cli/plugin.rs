// MIT License
//
// Copyright (c) 2024 Robin Doer
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
pub mod info;
pub mod list;
pub mod modify;
pub mod remove;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::cli::ctx::GlobalContext;
use crate::cli::plugin::add::PluginAddArgs;
use crate::cli::plugin::info::PluginInfoArgs;
use crate::cli::plugin::list::PluginListArgs;
use crate::cli::plugin::modify::PluginModifyArgs;
use crate::cli::plugin::remove::PluginRemoveArgs;

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct PluginArgs {
    #[clap(subcommand)]
    command: Option<PluginCommand>,
}

impl PluginArgs {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        self.command
            .as_ref()
            .map_or(Ok(()), |command| command.run(ctx))
    }
}

#[derive(Debug, Subcommand)]
pub enum PluginCommand {
    /// Assigns a new plugin
    Add(PluginAddArgs),

    /// Assigns a new plugin
    Modify(PluginModifyArgs),

    /// Removes a plugin again
    Remove(PluginRemoveArgs),

    /// Prints information about a plugin
    Info(PluginInfoArgs),

    /// Lists all configured plugins
    List(PluginListArgs),
}

impl PluginCommand {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        match self {
            Self::Add(args) => args.run(ctx),
            Self::Modify(args) => args.run(ctx),
            Self::Remove(args) => args.run(ctx),
            Self::Info(args) => args.run(ctx),
            Self::List(args) => args.run(ctx),
        }
    }
}
