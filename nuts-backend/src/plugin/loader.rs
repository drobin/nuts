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

use libloading::{Library, Symbol};
use log::{debug, trace};
use std::ffi::OsStr;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::{error, fmt, fs, io, result};

use crate::plugin::Plugin;

type PluginCreate = unsafe fn() -> *mut dyn Plugin;

#[derive(Debug)]
pub enum LoaderError {
    NoSuchPlugin(String),
    Libloading(libloading::Error),
    Io(io::Error),
}

impl fmt::Display for LoaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoaderError::NoSuchPlugin(name) => write!(fmt, "no such plugin: {}", name),
            LoaderError::Libloading(cause) => fmt::Display::fmt(cause, fmt),
            LoaderError::Io(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl error::Error for LoaderError {}

impl From<libloading::Error> for LoaderError {
    fn from(cause: libloading::Error) -> Self {
        LoaderError::Libloading(cause)
    }
}

impl From<io::Error> for LoaderError {
    fn from(cause: io::Error) -> Self {
        LoaderError::Io(cause)
    }
}

pub struct PluginLoader {
    path: PathBuf,
    _library: Library,
    plugin: Box<dyn Plugin>,
}

impl PluginLoader {
    pub fn is_plugin<P: AsRef<OsStr>>(path: P) -> bool {
        unsafe {
            match Library::new(path) {
                Ok(library) => Self::load_symbol(&library).is_ok(),
                Err(_) => false,
            }
        }
    }

    pub fn load<P: AsRef<OsStr>>(path: P) -> result::Result<PluginLoader, LoaderError> {
        unsafe { Self::load_unsafe(path.as_ref()) }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    unsafe fn load_unsafe(path: &OsStr) -> result::Result<PluginLoader, LoaderError> {
        let library = Library::new(path)?;
        let func = Self::load_symbol(&library)?;
        let boxed_raw = func();

        let plugin = Box::from_raw(boxed_raw);

        Ok(PluginLoader {
            path: Path::new(path).into(),
            _library: library,
            plugin,
        })
    }

    unsafe fn load_symbol(library: &Library) -> result::Result<Symbol<PluginCreate>, LoaderError> {
        Ok(library.get(b"_plugin_create\0")?)
    }
}

impl Deref for PluginLoader {
    type Target = Box<dyn Plugin>;

    fn deref(&self) -> &Self::Target {
        &self.plugin
    }
}

impl DerefMut for PluginLoader {
    fn deref_mut(&mut self) -> &mut Box<dyn Plugin> {
        &mut self.plugin
    }
}

fn traverse_plugins<T, CB: FnMut(&mut T, PluginLoader) -> bool>(
    search_path: &[String],
    mut value: T,
    mut callback: CB,
) -> result::Result<T, LoaderError> {
    for parent in search_path {
        trace!("testing directory {:?}", parent);

        for result in fs::read_dir(parent)? {
            let entry = result?;

            if entry.file_type()?.is_file() {
                if PluginLoader::is_plugin(entry.path()) {
                    debug!("plugin detected in {:?}", entry.file_name());

                    let loader = PluginLoader::load(entry.path())?;
                    let cont = callback(&mut value, loader);

                    if !cont {
                        return Ok(value);
                    }
                } else {
                    trace!("skipping file {:?}", entry.file_name());
                }
            } else {
                trace!("skipping non-file {:?}", entry.file_name());
            }
        }
    }

    Ok(value)
}

pub fn locate_plugins(search_path: &[String]) -> result::Result<Vec<PluginLoader>, LoaderError> {
    traverse_plugins(search_path, vec![], |vec, loader| {
        vec.push(loader);
        true
    })
}

pub fn locate_backend<N: AsRef<str>>(
    name: N,
    search_path: &[String],
) -> result::Result<PluginLoader, LoaderError> {
    traverse_plugins(search_path, None, |value, loader| {
        if loader.name() == name.as_ref() {
            let _ = value.insert(loader);
            false
        } else {
            true
        }
    })?
    .ok_or_else(|| LoaderError::NoSuchPlugin(name.as_ref().to_string()))
}
