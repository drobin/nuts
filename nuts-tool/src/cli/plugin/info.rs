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
use nuts_tool_api::tool::Plugin;

use crate::cli::ctx::GlobalContext;
use crate::config::PluginConfig;
use crate::say::say;

#[derive(Args, Debug)]
pub struct PluginInfoArgs {
    /// The name of the plugin
    name: String,
}

impl PluginInfoArgs {
    pub fn run(&self, ctx: &GlobalContext) -> Result<()> {
        debug!("args: {:?}", self);

        let config = PluginConfig::load()?;

        if !config.have_plugin(&self.name) {
            bail!("the plugin '{}' is not configured", self.name);
        }

        let path = config.path(&self.name)?;
        let plugin = Plugin::new(&path);
        let info = plugin.info()?;

        say!(ctx, "path:     {}", path.display());
        say!(ctx, "name:     {}", info.name());
        say!(ctx, "version:  {}", info.version());
        say!(ctx, "revision: {}", info.revision());

        Ok(())
    }
}
