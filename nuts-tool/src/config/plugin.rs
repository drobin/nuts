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
use is_executable::IsExecutable;
use log::{debug, error};
use nuts_tool_api::tool::Plugin;
use nuts_tool_api::tool_dir;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::config::load_path;

#[derive(Debug, Deserialize, Serialize)]
struct Inner {
    path: PathBuf,
}

impl Inner {
    fn absolute_path(&self) -> Result<Cow<Path>> {
        if self.path.is_absolute() {
            Ok(Cow::Borrowed(self.path.as_path()))
        } else {
            let cur_exe = env::current_exe()
                .map_err(|err| anyhow!("could not detect path of executable: {}", err))?;
            let path = cur_exe.with_file_name(self.path.as_os_str());

            debug!(
                "make absolute path from '{}' & '{}': '{}'",
                cur_exe.display(),
                self.path.display(),
                path.display()
            );

            Ok(Cow::Owned(path))
        }
    }

    fn validate(&self) -> bool {
        match self.absolute_path() {
            Ok(path) => Self::validate_path(&path),
            Err(err) => {
                error!("failed to validate {}: {}", self.path.display(), err);
                false
            }
        }
    }

    fn validate_path(path: &Path) -> bool {
        if !path.is_file() {
            error!("{}: not a file", path.display());
            return false;
        }

        if !path.is_executable() {
            error!("{}: not executable", path.display());
            return false;
        }

        let plugin = Plugin::new(&path);

        if let Err(err) = plugin.info() {
            error!("{}: not a plugin ({})", path.display(), err);
            return false;
        }

        debug!("{}: is valid", path.display());

        true
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginConfig {
    #[serde(flatten)]
    plugins: HashMap<String, Inner>,
}

impl PluginConfig {
    pub fn load() -> Result<PluginConfig> {
        match load_path(Self::config_file()?.as_path())? {
            Some(s) => Ok(toml::from_str(&s)?),
            None => Ok(PluginConfig {
                plugins: HashMap::new(),
            }),
        }
    }

    pub fn config_file() -> Result<PathBuf> {
        Ok(tool_dir()?.join("plugins"))
    }

    pub fn all_plugins(&self) -> Vec<&str> {
        let mut keys = self
            .plugins
            .iter()
            .filter(|(_, inner)| inner.validate())
            .map(|(name, _)| name.as_str())
            .collect::<Vec<&str>>();

        keys.sort();

        keys
    }

    pub fn have_plugin(&self, name: &str) -> bool {
        self.plugins
            .get(name)
            .filter(|inner| inner.validate())
            .is_some()
    }

    pub fn remove_plugin(&mut self, name: &str) -> bool {
        self.plugins.remove(name).is_some()
    }

    pub fn path(&self, name: &str) -> Result<Cow<Path>> {
        match self.plugins.get(name).filter(|inner| inner.validate()) {
            Some(inner) => inner.absolute_path(),
            None => Err(anyhow!("no such plugin: {}", name)),
        }
    }

    pub fn set_path<P: AsRef<Path>>(&mut self, name: &str, path: P) -> bool {
        let inner = Inner {
            path: path.as_ref().into(),
        };

        let valid = inner.validate();

        if valid {
            self.plugins.insert(name.to_string(), inner);
        }

        valid
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_file()?;
        let toml = toml::to_string(self)?;

        debug!("{}: dump {} bytes", path.display(), toml.as_bytes().len());

        fs::write(path, toml)?;

        Ok(())
    }
}
