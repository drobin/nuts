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

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use nuts_backend::plugin::{self, PluginLoader};
use nuts_backend::{Backend, Options};

pub struct ProxyCreateOptions {
    loader: Rc<RefCell<PluginLoader>>,
    path: PathBuf,
}

impl ProxyCreateOptions {
    pub fn new<P: AsRef<Path>>(loader: PluginLoader, path: P) -> ProxyCreateOptions {
        ProxyCreateOptions {
            loader: Rc::new(RefCell::new(loader)),
            path: path.as_ref().into(),
        }
    }
}

impl Options<ProxyBackend> for ProxyCreateOptions {
    fn validate(&self) -> plugin::Result<()> {
        Ok(())
    }
}

pub struct ProxyOpenOptions {
    loader: Rc<RefCell<PluginLoader>>,
    path: PathBuf,
}

impl ProxyOpenOptions {
    pub fn new<P: AsRef<Path>>(loader: PluginLoader, path: P) -> ProxyOpenOptions {
        ProxyOpenOptions {
            loader: Rc::new(RefCell::new(loader)),
            path: path.as_ref().into(),
        }
    }
}

impl Options<ProxyBackend> for ProxyOpenOptions {
    fn validate(&self) -> plugin::Result<()> {
        Ok(())
    }
}

pub struct ProxyBackend(Rc<RefCell<PluginLoader>>);

impl Options<ProxyBackend> for plugin::CreateOptions {
    fn validate(&self) -> plugin::Result<()> {
        unimplemented!()
    }
}

impl Backend for ProxyBackend {
    type CreateOptions = ProxyCreateOptions;
    type OpenOptions = ProxyOpenOptions;
    type Settings = plugin::Settings;
    type Err = plugin::Error;
    type Id = plugin::Id;
    type Info = plugin::Info;

    fn create(options: ProxyCreateOptions) -> plugin::Result<(Self, plugin::Settings)> {
        let mut loader = options.loader.borrow_mut();

        let plugin_options = loader.create_options(&options.path);
        let settings = loader.create(plugin_options)?;

        let backend = ProxyBackend(options.loader.clone());

        Ok((backend, settings))
    }

    fn open(options: ProxyOpenOptions) -> plugin::Result<Self> {
        let mut loader = options.loader.borrow_mut();

        let plugin_options = loader.open_options(&options.path);
        loader.open(plugin_options)?;

        let backend = ProxyBackend(options.loader.clone());

        Ok(backend)
    }

    fn open_ready(&mut self, settings: plugin::Settings) {
        // FIXME unwrap()...
        self.0.borrow_mut().open_ready(settings).unwrap();
    }

    fn info(&self) -> plugin::Result<plugin::Info> {
        self.0.borrow().info()
    }

    fn block_size(&self) -> u32 {
        // FIXME unwrap()...
        self.0.borrow().block_size().unwrap()
    }

    fn header_id(&self) -> plugin::Id {
        // FIXME unwrap()...
        self.0.borrow().header_id().unwrap()
    }

    fn aquire(&mut self) -> plugin::Result<plugin::Id> {
        self.0.borrow_mut().aquire()
    }

    fn release(&mut self, id: plugin::Id) -> plugin::Result<()> {
        self.0.borrow_mut().release(id)
    }

    fn read(&mut self, id: &plugin::Id, buf: &mut [u8]) -> plugin::Result<usize> {
        self.0.borrow_mut().read(id, buf)
    }

    fn write(&mut self, id: &plugin::Id, buf: &[u8]) -> plugin::Result<usize> {
        self.0.borrow_mut().write(id, buf)
    }
}
