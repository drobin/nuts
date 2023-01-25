// MIT License
//
// Copyright (c) 2023 Robin Doer
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
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::tool::tool_dir;

fn parse_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let content = if path.is_file() {
        debug!("read from {}", path.display());
        fs::read_to_string(path)?
    } else {
        debug!("no such file: {}", path.display());
        "".to_string()
    };

    Ok(toml::from_str(&content)?)
}

fn save_toml<T: serde::ser::Serialize>(path: &Path, obj: &T) -> Result<()> {
    let content = [
        "# Config file is generated automatically.",
        "\n",
        "# Any manual inserted content is replaced with the next write attempt!",
        "\n",
        "\n",
        &toml::to_string(obj)?,
    ]
    .concat();

    debug!("write config to {}", path.display());
    Ok(fs::write(path, content)?)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub search_path: Vec<String>,
}

impl Config {
    pub fn parse() -> Result<Config> {
        let file = tool_dir()?.join("config");
        debug!("config file: {}", file.display());

        parse_toml(&file)
    }

    pub fn save(&self) -> Result<()> {
        let file = tool_dir()?.join("config");
        debug!("config file: {}", file.display());

        save_toml(&file, self)
    }
}

#[derive(Serialize, Deserialize)]
struct Inner {
    pub backend: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContainerConfig(HashMap<String, Inner>);

impl ContainerConfig {
    fn parse() -> Result<ContainerConfig> {
        let file = tool_dir()?.join("container");
        debug!("container config file: {}", file.display());

        parse_toml(&file)
    }

    fn save(&self) -> Result<()> {
        let file = tool_dir()?.join("container");
        debug!("container config file: {}", file.display());

        save_toml(&file, self)
    }

    pub fn get_backend(name: &str) -> Result<String> {
        Self::parse()?.0.get(name).map_or_else(
            || Err(anyhow!("No such container: {}", name)),
            |inner| Ok(inner.backend.clone()),
        )
    }

    pub fn put_backend(name: &str, backend: &str) -> Result<()> {
        let mut config = ContainerConfig::parse()?;

        config.0.insert(
            name.to_string(),
            Inner {
                backend: backend.to_string(),
            },
        );
        config.save()?;

        Ok(())
    }

    pub fn remove_backend(name: &str) -> Result<()> {
        let mut config = ContainerConfig::parse()?;
        config.0.remove(name);
        config.save()
    }
}
