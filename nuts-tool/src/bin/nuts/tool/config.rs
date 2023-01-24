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

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::tool::tool_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub search_path: Vec<String>,
}

impl Config {
    pub fn parse() -> Result<Config> {
        let file = tool_dir()?.join("config");
        debug!("config file: {}", file.display());

        let content = if file.is_file() {
            debug!("read config from {}", file.display());
            fs::read_to_string(file)?
        } else {
            debug!("no such file: {}", file.display());
            "".to_string()
        };

        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let file = tool_dir()?.join("config");
        debug!("config file: {}", file.display());

        let content = [
            "# Config file is generated automatically.",
            "\n",
            "# Any manual inserted content is replaced with the next write attempt!",
            "\n",
            "\n",
            &toml::to_string(self)?,
        ]
        .concat();

        debug!("write config to {}", file.display());
        Ok(fs::write(file, content)?)
    }
}
