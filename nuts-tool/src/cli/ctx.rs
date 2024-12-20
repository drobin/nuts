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

use anyhow::{anyhow, Result};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use nuts_archive::{Archive, ArchiveFactory};
use nuts_container::{Container, OpenOptionsBuilder};
use nuts_tool_api::tool::Plugin;
use std::ops::Deref;

use crate::backend::{PluginBackend, PluginBackendOpenBuilder};
use crate::cli::archive::GlobalArchiveArgs;
use crate::cli::password::{password_from_source, PasswordSource};
use crate::cli::{GlobalArgs, GlobalContainerArgs};
use crate::config::{ContainerConfig, PluginConfig};

pub struct GlobalContext {
    pub verbose: u8,
    pub quiet: bool,
}

impl GlobalContext {
    pub fn new(args: &GlobalArgs) -> GlobalContext {
        GlobalContext {
            verbose: args.verbose,
            quiet: args.quiet,
        }
    }
}

pub struct ContainerContext<'a> {
    global: &'a GlobalContext,
    pub password_source: PasswordSource,
    container: Option<String>,
}

impl<'a> ContainerContext<'a> {
    pub fn new(parent: &'a GlobalContext, args: &GlobalContainerArgs) -> ContainerContext<'a> {
        ContainerContext {
            global: parent,
            password_source: PasswordSource::new(
                args.password_from_fd,
                args.password_from_file.clone(),
            ),
            container: args.container.clone(),
        }
    }

    pub fn container_name(&self) -> Result<&str> {
        self.container.as_ref().map(|s| s.as_str()).ok_or_else(|| {
            let mut err = clap::Error::new(ErrorKind::InvalidValue);

            err.insert(
                ContextKind::InvalidArg,
                ContextValue::String("--container".to_string()),
            );
            err.insert(
                ContextKind::InvalidValue,
                ContextValue::String("".to_string()),
            );

            err.into()
        })
    }

    pub fn open_container(&self) -> Result<Container<PluginBackend>> {
        let container_config = ContainerConfig::load()?;
        let plugin_config = PluginConfig::load()?;
        let name = self.container_name()?;

        let plugin = container_config
            .get_plugin(name)
            .ok_or_else(|| anyhow!("no such container: {}", name))?;
        let exe = plugin_config.path(plugin)?;

        let plugin = Plugin::new(&exe);
        let plugin_builder = PluginBackendOpenBuilder::new(plugin, name, self.verbose)?;

        let source = self.password_source.clone();
        let builder =
            OpenOptionsBuilder::new().with_password_callback(|| password_from_source(source));
        let options = builder.build::<PluginBackend>()?;

        Container::open(plugin_builder, options).map_err(|err| err.into())
    }
}

impl<'a> Deref for ContainerContext<'a> {
    type Target = GlobalContext;

    fn deref(&self) -> &GlobalContext {
        self.global
    }
}

pub struct ArchiveContext<'a> {
    container: &'a ContainerContext<'a>,
    migrate: bool,
}

impl<'a> ArchiveContext<'a> {
    pub fn new(parent: &'a ContainerContext, args: &GlobalArchiveArgs) -> ArchiveContext<'a> {
        ArchiveContext {
            container: parent,
            migrate: args.migrate,
        }
    }

    pub fn open_archive(&self) -> Result<Archive<PluginBackend>> {
        self.open_archive_args(self.migrate)
    }

    pub fn open_archive_args(&self, migrate: bool) -> Result<Archive<PluginBackend>> {
        let container = self.open_container()?;

        Container::open_service::<ArchiveFactory>(container, migrate).map_err(|err| err.into())
    }
}

impl<'a> Deref for ArchiveContext<'a> {
    type Target = ContainerContext<'a>;

    fn deref(&self) -> &ContainerContext<'a> {
        self.container
    }
}
