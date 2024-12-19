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
pub mod ctx;
pub mod error;
pub mod global;
pub mod password;
pub mod plugin;

use anyhow::Result;
use clap::{crate_version, ArgAction, Args, Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;
use rprompt::prompt_reply;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::cli::archive::ArchiveArgs;
use crate::cli::container::ContainerArgs;
use crate::cli::ctx::GlobalContext;
use crate::cli::plugin::PluginArgs;

#[derive(Args, Clone, Debug)]
pub struct GlobalArgs {
    /// Enable verbose output. Can be called multiple times
    #[clap(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Be quiet. Don't produce any output
    #[clap(short, long, action = ArgAction::SetTrue, global = true)]
    pub quiet: bool,
}

#[derive(Args, Clone, Debug)]
pub struct GlobalContainerArgs {
    /// Reads the password from the specified file descriptor <FD>. The
    /// password is the first line until a `\n` is read.
    #[clap(long, group = "password", global = true, value_name = "FD")]
    pub password_from_fd: Option<RawFd>,

    /// Reads the password from the specified file <PATH>. The password is the
    /// first line until a `\n` is read.
    #[clap(long, group = "password", global = true, value_name = "PATH")]
    pub password_from_file: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[clap(name = "nuts", bin_name = "nuts")]
#[clap(version = crate_version!())]
pub struct NutsCli {
    #[clap(subcommand)]
    command: Commands,

    #[command(flatten)]
    global_args: GlobalArgs,
}

impl NutsCli {
    pub fn configure_logging(&self) {
        let filter = match self.global_args.verbose {
            0 => LevelFilter::Off,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Builder::new().filter_level(filter).init();
    }

    pub fn run(&self) -> Result<()> {
        let ctx = GlobalContext::new(&self.global_args);

        self.command.run(&ctx)
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Configure plugins
    Plugin(PluginArgs),

    /// General container tasks
    Container(ContainerArgs),

    /// An archive on top of the container
    Archive(ArchiveArgs),
}

impl Commands {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        match self {
            Self::Plugin(args) => args.run(ctx),
            Self::Container(args) => args.run(ctx),
            Self::Archive(args) => args.run(ctx),
        }
    }
}

pub fn prompt_yes_no(prompt: &str, force: bool) -> Result<bool> {
    let ok = force || {
        let msg = format!("{} [yes/NO] ", prompt);
        let reply = prompt_reply(msg)?;

        reply == "yes"
    };

    Ok(ok)
}
