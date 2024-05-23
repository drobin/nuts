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

use log::error;
use nuts_backend::Backend;
use nuts_directory::{CreateOptions, DirectoryBackend, Info, OpenOptions};
use nuts_tool_api::plugin::clap_prelude::*;
use nuts_tool_api::plugin::cli::{CreateArgs, OpenArgs};
use nuts_tool_api::plugin::{PluginHandler, PluginRunner};
use nuts_tool_api::{container_dir_for, ErrorResponse, PluginInfo};
use std::{collections::HashMap, path::PathBuf, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Args, Debug)]
struct ExtraArgs {
    /// Set the block-size to SIZE
    #[clap(short, long, id = "SIZE", default_value = "512")]
    block_size: u32,
}

struct DirectoryPluginInformation;

impl PluginHandler<DirectoryBackend<PathBuf>> for DirectoryPluginInformation {
    type CreateArgs = ExtraArgs;
    type Create = CreateOptions<PathBuf>;
    type Open = OpenOptions<PathBuf>;

    fn plugin_info(&self) -> PluginInfo {
        PluginInfo::new("directory", VERSION)
    }

    fn info_to_hash(&self, info: Info) -> Option<HashMap<String, String>> {
        Some([("block_size".to_string(), info.bsize.to_string())].into())
    }

    fn open_builder(&self, args: &OpenArgs) -> Option<OpenOptions<PathBuf>> {
        match container_dir_for(&args.name) {
            Ok(path) => Some(OpenOptions::for_path(path)),
            Err(err) => {
                error!("could not detect container dir for {}: {}", args.name, err);
                None
            }
        }
    }

    fn create_builder(&self, args: &CreateArgs<ExtraArgs>) -> Option<CreateOptions<PathBuf>> {
        match container_dir_for(&args.name) {
            Ok(path) => Some(CreateOptions::for_path(path).with_bsize(args.extra.block_size)),
            Err(err) => {
                error!("could not detect container dir for {}: {}", args.name, err);
                None
            }
        }
    }

    fn handle_info(
        &self,
        backend: &DirectoryBackend<PathBuf>,
    ) -> Result<HashMap<String, String>, ErrorResponse> {
        match backend.info() {
            Ok(info) => Ok([("block_size".to_string(), info.bsize.to_string())].into()),
            Err(err) => Err(err.into()),
        }
    }
}

fn main() {
    let mut runner = PluginRunner::new(DirectoryPluginInformation);

    runner.configure_logging();

    process::exit(match runner.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    })
}
