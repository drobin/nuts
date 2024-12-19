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

pub mod aquire;
pub mod attach;
pub mod change;
pub mod create;
pub mod delete;
pub mod info;
pub mod list;
pub mod read;
pub mod release;
pub mod write;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, Subcommand, ValueEnum};
use nuts_container::Cipher;
use std::ops::Deref;

use crate::cli::container::aquire::ContainerAquireArgs;
use crate::cli::container::attach::ContainerAttachArgs;
use crate::cli::container::change::ContainerChangeArgs;
use crate::cli::container::create::ContainerCreateArgs;
use crate::cli::container::delete::ContainerDeleteArgs;
use crate::cli::container::info::ContainerInfoArgs;
use crate::cli::container::list::ContainerListArgs;
use crate::cli::container::read::ContainerReadArgs;
use crate::cli::container::release::ContainerReleaseArgs;
use crate::cli::container::write::ContainerWriteArgs;
use crate::cli::ctx::{ContainerContext, GlobalContext};
use crate::cli::GlobalContainerArgs;

const AES128_GCM: &str = "aes128-gcm";
const AES192_GCM: &str = "aes192-gcm";
const AES256_GCM: &str = "aes256-gcm";
const AES128_CTR: &str = "aes128-ctr";
const AES192_CTR: &str = "aes192-ctr";
const AES256_CTR: &str = "aes256-ctr";
const NONE: &str = "none";

#[derive(Clone, Debug)]
pub struct CliCipher(Cipher);

impl PartialEq<Cipher> for CliCipher {
    fn eq(&self, other: &Cipher) -> bool {
        self.0 == *other
    }
}

impl Deref for CliCipher {
    type Target = Cipher;

    fn deref(&self) -> &Cipher {
        &self.0
    }
}

impl ValueEnum for CliCipher {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CliCipher(Cipher::Aes128Gcm),
            CliCipher(Cipher::Aes192Gcm),
            CliCipher(Cipher::Aes256Gcm),
            CliCipher(Cipher::Aes192Ctr),
            CliCipher(Cipher::Aes256Ctr),
            CliCipher(Cipher::Aes128Ctr),
            CliCipher(Cipher::None),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let value = match self.0 {
            Cipher::None => NONE,
            Cipher::Aes128Ctr => AES128_CTR,
            Cipher::Aes192Ctr => AES192_CTR,
            Cipher::Aes256Ctr => AES256_CTR,
            Cipher::Aes128Gcm => AES128_GCM,
            Cipher::Aes192Gcm => AES192_GCM,
            Cipher::Aes256Gcm => AES256_GCM,
        };

        Some(PossibleValue::new(value))
    }
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct ContainerArgs {
    #[clap(subcommand)]
    command: Option<ContainerCommand>,

    #[command(flatten)]
    global_args: GlobalContainerArgs,
}

impl ContainerArgs {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        let ctx = ContainerContext::new(ctx, &self.global_args);

        self.command
            .as_ref()
            .map_or(Ok(()), |command| command.run(&ctx))
    }
}

#[derive(Debug, Subcommand)]
pub enum ContainerCommand {
    /// Aquires a new block in a container
    Aquire(ContainerAquireArgs),

    /// Attaches a plugin to a nuts-container
    Attach(ContainerAttachArgs),

    /// Modifies the container
    Change(ContainerChangeArgs),

    /// Creates a nuts-container
    Create(ContainerCreateArgs),

    /// Removes a container again
    Delete(ContainerDeleteArgs),

    /// Prints general information about the container
    Info(ContainerInfoArgs),

    /// Lists all available container
    List(ContainerListArgs),

    /// Reads a block from the container
    Read(ContainerReadArgs),

    /// Releases a block again
    Release(ContainerReleaseArgs),

    /// Writes a block into the container
    Write(ContainerWriteArgs),
}

impl ContainerCommand {
    pub fn run(&self, ctx: &ContainerContext) -> Result<()> {
        match self {
            Self::Aquire(args) => args.run(ctx),
            Self::Attach(args) => args.run(ctx),
            Self::Change(args) => args.run(ctx),
            Self::Create(args) => args.run(ctx),
            Self::Delete(args) => args.run(ctx),
            Self::Info(args) => args.run(ctx),
            Self::List(args) => args.run(ctx),
            Self::Read(args) => args.run(ctx),
            Self::Release(args) => args.run(ctx),
            Self::Write(args) => args.run(ctx),
        }
    }
}
