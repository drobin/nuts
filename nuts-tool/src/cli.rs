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
pub mod plugin;

use anyhow::anyhow;
use anyhow::Result;
use clap::{crate_version, ArgAction, Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;
use nuts_container::{Container, OpenOptionsBuilder};
use nuts_tool_api::tool::Plugin;
use rpassword::prompt_password;

use crate::backend::{PluginBackend, PluginBackendOpenBuilder};
use crate::cli::archive::ArchiveArgs;
use crate::cli::container::ContainerArgs;
use crate::cli::plugin::PluginArgs;
use crate::config::{ContainerConfig, PluginConfig};

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
    /// Configure plugins
    Plugin(PluginArgs),

    /// General container tasks
    Container(ContainerArgs),

    /// An archive on top of the container
    Archive(ArchiveArgs),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Plugin(args) => args.run(),
            Self::Container(args) => args.run(),
            Self::Archive(args) => args.run(),
        }
    }
}

fn open_container(name: &str, verbose: u8) -> Result<Container<PluginBackend>> {
    let container_config = ContainerConfig::load()?;
    let plugin_config = PluginConfig::load()?;

    let plugin = container_config
        .get_plugin(name)
        .ok_or_else(|| anyhow!("no such container: {}", name))?;
    let exe = plugin_config.path(plugin)?;

    let plugin = Plugin::new(&exe);
    let plugin_builder = PluginBackendOpenBuilder::new(plugin, name, verbose)?;

    let builder = OpenOptionsBuilder::new().with_password_callback(ask_for_password);
    let options = builder.build::<PluginBackend>()?;

    Container::open(plugin_builder, options).map_err(|err| err.into())
}

pub fn ask_for_password() -> Result<Vec<u8>, String> {
    let password = prompt_password("Enter a password: ").map_err(|err| err.to_string())?;
    Ok(password.as_bytes().to_vec())
}
