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

//! Command line interface for the plugin.

use clap::{crate_version, ArgAction, Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Format {
    Text,
    Bson,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    #[clap(long, default_value = "text")]
    pub format: Format,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    /// Name of the container
    pub name: String,
}

#[derive(Args, Debug)]
pub struct CreateArgs<CX: Args> {
    /// Name of the container
    pub name: String,

    #[clap(flatten)]
    pub extra: CX,
}

#[derive(Debug, Subcommand)]
pub enum PluginCommand<CX: Args> {
    /// Prints information about the plugin
    Info(InfoArgs),

    /// Opens a backend instance
    Open(OpenArgs),

    /// Creates a new backend instance
    Create(CreateArgs<CX>),
}

impl<CX: Args> PluginCommand<CX> {
    pub fn as_info(&self) -> Option<&InfoArgs> {
        match self {
            Self::Info(args) => Some(args),
            _ => None,
        }
    }

    pub fn as_open(&self) -> Option<&OpenArgs> {
        match self {
            Self::Open(args) => Some(args),
            _ => None,
        }
    }

    pub fn as_create(&self) -> Option<&CreateArgs<CX>> {
        match self {
            Self::Create(args) => Some(args),
            _ => None,
        }
    }
}

#[derive(Debug, Parser)]
#[clap(version = crate_version!())]
pub struct PluginCli<CX: Args> {
    #[clap(subcommand)]
    pub command: PluginCommand<CX>,

    /// Enable verbose output. Can be called multiple times
    #[clap(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,
}
