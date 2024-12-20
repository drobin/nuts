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

use anyhow::Result;
use clap::{ArgAction, Args};
use log::debug;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::cli::ctx::ContainerContext;
use crate::config::{ContainerConfig, PluginConfig};
use crate::say::{say, say_warn};

#[derive(Args, Debug)]
pub struct ContainerListArgs {
    /// Display all container (even with broken configuration)
    #[clap(short, long, action = ArgAction::SetTrue)]
    all: bool,

    #[clap(long, hide = true)]
    password_from_fd: Option<RawFd>,

    #[clap(long, hide = true)]
    password_from_file: Option<PathBuf>,

    #[clap(long, hide = true)]
    container: Option<String>,
}

impl ContainerListArgs {
    pub fn run(&self, ctx: &ContainerContext) -> Result<()> {
        debug!("args: {:?}", self);

        let container_config = ContainerConfig::load()?;
        let plugin_config = PluginConfig::load()?;

        for name in container_config.list_container() {
            let ok = container_config
                .get_plugin(name)
                .map(|p| plugin_config.have_plugin(p))
                .unwrap_or(false);

            if ok && self.all {
                say!(ctx, "  {}", name);
            } else if ok && !self.all {
                say!(ctx, "{}", name);
            } else if self.all {
                say_warn!(ctx, "! {}", name);
            }
        }

        Ok(())
    }
}
