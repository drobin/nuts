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
use is_executable::IsExecutable;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::plugin::Plugin;
use crate::tool_dir;

fn load_path(path: &Path) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(s) => {
            debug!("{}: {} bytes read", path.display(), s.as_bytes().len());
            Ok(Some(s))
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                debug!("{}: not found", path.display());
                Ok(None)
            } else {
                error!("{}: {}", path.display(), err);
                Err(err)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Inner {
    path: PathBuf,
}

impl Inner {
    fn validate(&self) -> bool {
        if !self.path.is_file() {
            error!("{}: not a file", self.path.display());
            return false;
        }

        if !self.path.is_executable() {
            error!("{}: not executable", self.path.display());
            return false;
        }

        let plugin = Plugin::new(&self.path);

        if let Err(err) = plugin.info() {
            error!("{}: not a plugin ({})", self.path.display(), err);
            return false;
        }

        debug!("{}: is valid", self.path.display());

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

    pub fn path(&self, name: &str) -> Option<&Path> {
        self.plugins
            .get(name)
            .filter(|inner| inner.validate())
            .map(|inner| inner.path.as_path())
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
