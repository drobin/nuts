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

use anyhow::{ensure, Result};
use clap::{value_parser, ArgAction, Args};
use log::debug;
use nuts_container::{Cipher, Container, CreateOptionsBuilder, Kdf};
use nuts_tool_api::tool::Plugin;
use std::os::fd::RawFd;
use std::path::PathBuf;

use crate::backend::{PluginBackend, PluginBackendCreateBuilder};
use crate::cli::container::{CliCipher, AES256_GCM};
use crate::cli::ctx::ContainerContext;
use crate::cli::password::{
    new_password_from_source_twice as password_from_source_twice, PasswordSource,
};
use crate::config::{ContainerConfig, PluginConfig};

#[derive(Args, Debug)]
pub struct ContainerCreateArgs {
    /// The  name of the new container
    name: String,

    /// Specifies the plugin used by the new container
    #[clap(short, long)]
    plugin: String,

    /// Sets the cipher to CIPHER.
    #[clap(short, long, value_parser = value_parser!(CliCipher), default_value = AES256_GCM)]
    cipher: CliCipher,

    /// Specifies the key derivation function.
    ///
    /// There are two ways to specify the KDF. The short form
    /// only specifies the algorithm name. The long form can
    /// customize the algorithm; it starts with the algorithm
    /// name followed by sections separated by a colon. A section
    /// can empty. In this case a default value is taken. The
    /// number of sections and its meaning depends on the
    /// algorithm.
    ///
    /// For PBKDF2: pbkdf2[:[<DIGEST>]:[<ITERATIONS>]:[<SALT_LENGTH>]]
    ///
    /// Selects PBKDF2 with the given digest (default: sha256),
    /// the given number of iterations (default: 65536) and salt
    /// length (default: 16).
    #[clap(short, long, value_parser)]
    kdf: Option<Kdf>,

    /// If set, overwrites an existing container
    #[clap(short, long, action = ArgAction::SetTrue)]
    overwrite: bool,

    /// Arguments passed to the plugin
    #[clap(value_name = "PLUGIN ARGS")]
    plugin_args: Vec<String>,

    #[clap(from_global)]
    verbose: u8,

    #[clap(from_global)]
    password_from_fd: Option<RawFd>,

    #[clap(from_global)]
    password_from_file: Option<PathBuf>,
}

impl ContainerCreateArgs {
    pub fn run(&self, _ctx: &ContainerContext) -> Result<()> {
        debug!("args: {:?}", self);

        let plugin_config = PluginConfig::load()?;
        let mut container_config = ContainerConfig::load()?;

        let exe = plugin_config.path(&self.plugin)?;
        let plugin = Plugin::new(&exe);

        let ok = container_config.add_plugin(&self.name, &self.plugin, self.overwrite);
        ensure!(
            ok,
            "you already have a container with the name {}",
            self.name
        );

        let source = PasswordSource::new(self.password_from_fd, self.password_from_file.clone());

        let backend_options =
            PluginBackendCreateBuilder::new(plugin, &self.name, self.verbose, &self.plugin_args)?;
        let mut builder = CreateOptionsBuilder::new(*self.cipher)
            .with_password_callback(|| password_from_source_twice(source, "Enter a password"))
            .with_overwrite(self.overwrite);

        if self.cipher != Cipher::None {
            if let Some(kdf) = self.kdf.clone() {
                debug!("kdf: {:?}", kdf);
                builder = builder.with_kdf(kdf);
            }
        }

        let options = builder.build::<PluginBackend>()?;
        Container::<PluginBackend>::create(backend_options, options)?;

        container_config.save()?;

        Ok(())
    }
}
