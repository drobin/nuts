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

use anyhow::Result;
use log::debug;
use nuts_tool_api::tool_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::load_path;

#[derive(Debug, Deserialize, Serialize)]
struct Inner {
    plugin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerConfig {
    #[serde(flatten)]
    container: HashMap<String, Inner>,
}

impl ContainerConfig {
    pub fn config() -> Result<PathBuf> {
        Ok(tool_dir()?.join("container"))
    }

    pub fn load() -> Result<ContainerConfig> {
        let path = Self::config()?;

        match load_path(&path)? {
            Some(s) => Ok(toml::from_str(&s)?),
            None => Ok(ContainerConfig {
                container: HashMap::new(),
            }),
        }
    }

    pub fn list_container(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = self.container.keys().map(|s| s.as_str()).collect();

        vec.sort();

        vec
    }

    pub fn get_plugin(&self, name: &str) -> Option<&str> {
        self.container.get(name).map(|inner| inner.plugin.as_str())
    }

    pub fn add_plugin(&mut self, name: &str, plugin: &str, force: bool) -> bool {
        if self.container.contains_key(name) && !force {
            return false;
        }

        self.container.insert(
            name.to_string(),
            Inner {
                plugin: plugin.to_string(),
            },
        );

        true
    }

    pub fn remove_plugin(&mut self, name: &str) -> bool {
        self.container.remove(name).is_some()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config()?;
        let toml = toml::to_string(self)?;

        debug!("{}: dump {} bytes", path.display(), toml.as_bytes().len());

        fs::write(path, toml)?;

        Ok(())
    }
}
