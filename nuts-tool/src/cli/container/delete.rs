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
use nuts_tool_api::container_dir_for;
use std::fs;

use crate::cli::ctx::{say, say_warn, ContainerContext};
use crate::cli::prompt_yes_no;
use crate::config::ContainerConfig;

#[derive(Args, Debug)]
pub struct ContainerDeleteArgs {
    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    /// Say yes, don't prompt for deletion
    #[clap(short, long, action = ArgAction::SetTrue)]
    yes: bool,

    /// Enforces the deletion. Removes the container without connecting to it.
    /// Note that depending on the backend, data may remain.
    #[clap(short, long, action = ArgAction::SetTrue)]
    force: bool,
}

impl ContainerDeleteArgs {
    pub fn run(&self, ctx: &ContainerContext) -> Result<()> {
        debug!("args: {:?}", self);

        if !prompt_yes_no("Do you really want to delete the container?", self.yes)? {
            say!(ctx, "aborted");
            return Ok(());
        }

        let path = container_dir_for(&self.container)?;
        let mut container_config = ContainerConfig::load()?;

        debug!("container: {}", self.container);
        debug!("path: {}", path.display());

        if !container_config.remove_plugin(&self.container) {
            say_warn!(ctx, "container {} not configured", self.container);
        }

        if !self.force {
            let container = ctx.open_container(&self.container)?;
            container.delete();
        }

        if path.exists() {
            fs::remove_dir_all(path)?;
        }

        container_config.save()?;

        Ok(())
    }
}
