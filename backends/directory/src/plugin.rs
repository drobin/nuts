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

use nuts_backend::plugin::{self, Info, Plugin};
use nuts_backend::{declare_plugin, Backend, Options};
use std::path::Path;

use crate::{DirectoryBackend, DirectoryCreateOptions, DirectoryOpenOptions};

pub struct DirectoryPlugin(Option<DirectoryBackend>);

macro_rules! into_backend_err {
    ($expr:expr) => {
        $expr.or_else(|cause| Err(plugin::Error::Backend(Box::new(cause))))
    };
}

impl DirectoryPlugin {
    pub fn new() -> DirectoryPlugin {
        DirectoryPlugin(None)
    }
}

impl Plugin for DirectoryPlugin {
    fn name(&self) -> &'static str {
        "directory"
    }

    fn create_options(&self, path: &Path) -> plugin::CreateOptions {
        let options = DirectoryCreateOptions::for_path(path);

        plugin::CreateOptions::new()
            .add("path", path.display().to_string())
            .add("bsize", options.bsize)
            .add("overwrite", options.overwrite)
    }

    fn open_options(&self, path: &Path) -> plugin::OpenOptions {
        let options = DirectoryOpenOptions::for_path(path);

        plugin::OpenOptions::new().add("path", options.path.display().to_string())
    }

    fn create(&mut self, options: plugin::CreateOptions) -> plugin::Result<plugin::Settings> {
        if self.0.is_some() {
            return Err(plugin::Error::Open);
        }

        let path = options.get_as_str("path")?;
        let bsize = options.get_as_u32("bsize")?;
        let overwrite = options.get_as_bool("overwrite")?;

        let options = DirectoryCreateOptions::for_path(path)
            .with_bsize(*bsize)
            .with_overwrite(*overwrite);
        into_backend_err!(options.validate())?;

        let (backend, settings) = into_backend_err!(DirectoryBackend::create(options))?;

        self.0 = Some(backend);

        Ok(plugin::Settings::to_bytes(&settings)?)
    }

    fn open(&mut self, options: plugin::OpenOptions) -> plugin::Result<()> {
        if self.0.is_some() {
            return Err(plugin::Error::Open);
        }

        let path = options.get_as_str("path")?;

        let options = DirectoryOpenOptions::for_path(path);

        into_backend_err!(options.validate())?;

        self.0 = Some(into_backend_err!(DirectoryBackend::open(options))?);

        Ok(())
    }

    fn open_ready(&mut self, settings: plugin::Settings) -> plugin::Result<()> {
        match self.0.as_mut() {
            Some(backend) => {
                let settings = settings.from_bytes()?;

                Ok(backend.open_ready(settings))
            }
            None => Err(plugin::Error::Closed),
        }
    }

    fn info(&self) -> plugin::Result<Info> {
        match self.0.as_ref() {
            Some(backend) => {
                let info = into_backend_err!(backend.info())?;
                Ok(Info::new().put("block size", info.bsize.to_string()))
            }
            None => Err(plugin::Error::Closed),
        }
    }

    fn block_size(&self) -> plugin::Result<u32> {
        match self.0.as_ref() {
            Some(backend) => Ok(backend.block_size()),
            None => Err(plugin::Error::Closed),
        }
    }

    fn header_id(&self) -> plugin::Result<plugin::Id> {
        match self.0.as_ref() {
            Some(backend) => Ok(backend.header_id().to_string().into()),
            None => Err(plugin::Error::Closed),
        }
    }

    fn aquire(&mut self) -> plugin::Result<plugin::Id> {
        match self.0.as_mut() {
            Some(backend) => {
                let id = into_backend_err!(backend.aquire())?;
                Ok(id.to_string().into())
            }
            None => Err(plugin::Error::Closed),
        }
    }

    fn release(&mut self, id: plugin::Id) -> plugin::Result<()> {
        match self.0.as_mut() {
            Some(backend) => {
                let id = into_backend_err!(id.into_string().parse())?;
                into_backend_err!(backend.release(id))
            }
            None => Err(plugin::Error::Closed),
        }
    }

    fn read(&mut self, id: &plugin::Id, buf: &mut [u8]) -> plugin::Result<usize> {
        match self.0.as_mut() {
            Some(backend) => {
                let id = into_backend_err!(id.as_str().parse())?;
                into_backend_err!(backend.read(&id, buf))
            }
            None => Err(plugin::Error::Closed),
        }
    }

    fn write(&mut self, id: &plugin::Id, buf: &[u8]) -> plugin::Result<usize> {
        match self.0.as_mut() {
            Some(backend) => {
                let id = into_backend_err!(id.as_str().parse())?;
                into_backend_err!(backend.write(&id, buf))
            }
            None => Err(plugin::Error::Closed),
        }
    }
}

declare_plugin!(DirectoryPlugin, DirectoryPlugin::new);
