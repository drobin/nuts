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

use anyhow::{anyhow, ensure, Result};
use clap::{ArgAction, Args};
use log::debug;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::cli::ctx::ContainerContext;
use crate::config::{ContainerConfig, PluginConfig};

#[derive(Args, Debug)]
pub struct ContainerAttachArgs {
    /// Attaches PLUGIN to CONTAINER
    plugin: String,

    /// Enforce the operation, even if a plugin is already attached to the
    /// container
    #[clap(short, long, action = ArgAction::SetTrue)]
    force: bool,

    #[clap(long, hide = true)]
    password_from_fd: Option<RawFd>,

    #[clap(long, hide = true)]
    password_from_file: Option<PathBuf>,
}

impl ContainerAttachArgs {
    pub fn run(&self, ctx: &ContainerContext) -> Result<()> {
        debug!("args: {:?}", self);

        let container_name = ctx.container_name()?;
        let mut container_config = ContainerConfig::load()?;
        let plugin_config = PluginConfig::load()?;

        ensure!(
            plugin_config.have_plugin(&self.plugin),
            "no such plugin: {}",
            self.plugin
        );

        if !container_config.add_plugin(container_name, &self.plugin, self.force) {
            return Err(anyhow!(
                "you already have a container with the name {}",
                container_name,
            ));
        }

        container_config.save()
    }
}
