// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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
use clap::Parser;
use nuts_tool::backend::PluginBackend;
use nuts_tool::cli::error::ExitOnly;
use nuts_tool::cli::NutsCli;
use nuts_tool::{say, say_err};

type ArchiveError = nuts_archive::Error<PluginBackend>;

fn print_archive_error(err: &ArchiveError) -> bool {
    match err {
        ArchiveError::UnsupportedRevision(rev, version) => {
            say_err!(
                "The archive is not supported anymore!\n\
                The latest version that supports the revision {} is {}.\n\
                Any newer version will no longer be able to read this archive.",
                rev,
                version
            );
            true
        }
        _ => false,
    }
}

fn handle_error(err: anyhow::Error) -> i32 {
    let mut exit_code = 1;
    let mut printed = false;

    if let Some(err) = err.downcast_ref::<ExitOnly>() {
        exit_code = err.code();
        printed = true;
    } else if let Some(err) = err.downcast_ref::<ArchiveError>() {
        printed = print_archive_error(err);
    }

    if !printed {
        say_err!("{}", err);
    }

    exit_code
}

fn main() -> Result<()> {
    std::process::exit(match run_cli() {
        Ok(_) => 0,
        Err(err) => handle_error(err),
    })
}

fn run_cli() -> Result<()> {
    let cli = NutsCli::parse();

    cli.configure_logging();
    cli.run()
}
