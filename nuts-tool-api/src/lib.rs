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

mod bson;
mod info;
mod msg;
#[cfg(feature = "plugin")]
pub mod plugin;
#[cfg(feature = "tool")]
pub mod tool;

use log::debug;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

pub use bson::{BsonError, BsonReader, BsonWriter};
pub use info::{PluginInfo, CURRENT_REVISION};
pub use msg::{ErrorResponse, OkResponse, Request, Response};

pub fn tool_dir() -> io::Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => {
            let tool_dir = dir.join(".nuts");

            debug!("tool_dir: {}", tool_dir.display());

            if !tool_dir.is_dir() {
                debug!("creating tool dir {}", tool_dir.display());
                fs::create_dir(&tool_dir)?;
            }

            Ok(tool_dir)
        }
        None => Err(io::Error::new(
            ErrorKind::NotFound,
            "unable to locate home-directory",
        )),
    }
}

pub fn container_dir() -> io::Result<PathBuf> {
    let parent = tool_dir()?;
    let dir = parent.join("container.d");

    debug!("container_dir: {}", dir.display());

    if !dir.is_dir() {
        debug!("creating container dir {}", dir.display());
        fs::create_dir(&dir)?;
    }

    Ok(dir)
}

pub fn container_dir_for<S: AsRef<str>>(name: S) -> io::Result<PathBuf> {
    let parent = container_dir()?;
    let dir = parent.join(name.as_ref());

    debug!("container_dir for {}: {}", name.as_ref(), dir.display());

    Ok(dir)
}
