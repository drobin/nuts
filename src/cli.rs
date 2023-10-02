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

pub mod container;

use anyhow::Result;
use clap::{ArgAction, Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;

use crate::cli::container::ContainerArgs;

const SHORT_VERSION: &str = env!("NUTS_TOOL_SHORT_VERSION");
const LONG_VERSION: &str = env!("NUTS_TOOL_LONG_VERSION");

#[derive(Debug, Parser)]
#[clap(name = "nuts", bin_name = "nuts")]
#[clap(version = SHORT_VERSION, long_version = LONG_VERSION)]
pub struct NutsCli {
    #[clap(subcommand)]
    command: Commands,

    /// Enable verbose output. Can be called multiple times
    #[clap(short, long, action = ArgAction::Count, global = true)]
    verbose: u8,
}

impl NutsCli {
    pub fn configure_logging(&self) {
        let filter = match self.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
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
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::Container(args) => args.run(),
        }
    }
}
