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
use clap::Args;
use log::debug;
use nuts_container::container::Container;
use nuts_directory::DirectoryBackend;
use std::path::PathBuf;

use crate::cli::open_container;
use crate::format::Format;

#[derive(Args, Debug)]
pub struct ContainerInfoArgs {
    /// If set, displays the userdata of the container
    #[clap(short, long)]
    userdata: bool,

    /// Specifies the format of the userdata dump
    #[clap(short, long, value_parser, default_value = "raw")]
    format: Format,

    /// Specifies the name of the container
    #[clap(short, long, env = "NUTS_CONTAINER")]
    container: String,
}

impl ContainerInfoArgs {
    fn print_info(&self, container: &Container<DirectoryBackend<PathBuf>>) -> Result<()> {
        let info = container.info()?;

        println!("cipher:             {}", info.cipher);
        println!("kdf:                {}", info.kdf.to_string());
        println!("block size (gross): {}", info.bsize_gross);
        println!("block size (net):   {}", info.bsize_net);

        Ok(())
    }

    fn print_userdata(&self, container: &Container<DirectoryBackend<PathBuf>>) -> Result<()> {
        let mut writer = self.format.create_writer();

        writer.print(container.userdata())?;
        writer.flush()?;

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        debug!("container: {}", self.container);
        debug!("userdata: {}", self.userdata);
        debug!("format: {:?}", self.format);

        let container = open_container(&self.container)?;

        if self.userdata {
            self.print_userdata(&container)
        } else {
            self.print_info(&container)
        }
    }
}
