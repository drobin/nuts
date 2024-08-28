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
use clap::Args;
use log::debug;
use std::cmp;

use crate::cli::open_container;
use crate::config::ContainerConfig;
use crate::format::Format;
use crate::say;

#[derive(Args, Debug)]
pub struct ContainerInfoArgs {
    /// Specifies the format of the userdata dump
    #[clap(short, long, value_parser, default_value = "raw")]
    format: Format,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,

    #[clap(from_global)]
    verbose: u8,
}

impl ContainerInfoArgs {
    pub fn run(&self) -> Result<()> {
        debug!("args: {:?}", self);

        let container = open_container(&self.container, self.verbose)?;
        let container_config = ContainerConfig::load()?;
        let plugin = container_config.get_plugin(&self.container).unwrap_or("?");
        let info = container.info()?;

        let key_width = 19;
        let key_width = info
            .backend
            .iter()
            .fold(key_width, |acc, (key, _)| cmp::max(acc, key.len() + 1));

        say!("{:<key_width$} {}", "plugin:", plugin);
        say!("{:<key_width$} {}", "revision:", info.revision);
        say!("{:<key_width$} {}", "cipher:", info.cipher);
        say!("{:<key_width$} {}", "kdf:", info.kdf.to_string());
        say!("{:<key_width$} {}", "block size (gross):", info.bsize_gross);
        say!("{:<key_width$} {}", "block size (net):", info.bsize_net);

        say!("");

        for (key, value) in info.backend {
            say!("{:<key_width$} {}", format!("{}:", key), value);
        }

        Ok(())
    }
}
