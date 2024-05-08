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

mod container;
mod plugin;

use log::{debug, error};
use std::path::Path;
use std::{fs, io};

pub use container::ContainerConfig;
pub use plugin::PluginConfig;

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
