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

mod error;
mod id;
mod info;
mod loader;
mod options;
mod settings;

use std::path::Path;

pub use error::{Error, Result};
pub use id::Id;
pub use info::Info;
pub use loader::{locate_backend, locate_plugins, LoaderError, PluginLoader};
pub use options::{CreateOptions, OpenOptions};
pub use settings::Settings;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn create_options(&self, path: &Path) -> CreateOptions;
    fn open_options(&self, path: &Path) -> OpenOptions;
    fn create(&mut self, options: CreateOptions) -> Result<Settings>;
    fn open(&mut self, options: OpenOptions) -> Result<()>;
    fn open_ready(&mut self, settings: Settings) -> Result<()>;
    fn info(&self) -> Result<Info>;
    fn block_size(&self) -> Result<u32>;
    fn header_id(&self) -> Result<Id>;
    fn aquire(&mut self) -> Result<Id>;
    fn release(&mut self, id: Id) -> Result<()>;
    fn read(&mut self, id: &Id, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, id: &Id, buf: &[u8]) -> Result<usize>;
}

#[macro_export]
macro_rules! declare_plugin {
    ($ty:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn nuts_backend::plugin::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $ty = $constructor;

            let object = constructor();
            let boxed: Box<dyn nuts_backend::plugin::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
