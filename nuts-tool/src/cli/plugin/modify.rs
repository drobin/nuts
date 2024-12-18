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

use anyhow::{bail, Result};
use clap::Args;
use log::debug;
use std::path::PathBuf;

use crate::cli::ctx::GlobalContext;
use crate::config::PluginConfig;

#[derive(Args, Debug)]
pub struct PluginModifyArgs {
    /// The name of the plugin
    name: String,

    /// The new location of the plugin
    #[clap(short, long)]
    path: Option<PathBuf>,
}

impl PluginModifyArgs {
    pub fn run(&self, _ctx: &GlobalContext) -> Result<()> {
        debug!("args: {:?}", self);

        let mut config = PluginConfig::load()?;

        if !config.have_plugin(&self.name) {
            bail!("the plugin '{}' is not configured", self.name);
        }

        if let Some(p) = self.path.as_ref() {
            if !config.set_path(&self.name, p) {
                bail!("the path '{}' is invalid", p.display());
            }
        }

        config.save()?;

        Ok(())
    }
}
