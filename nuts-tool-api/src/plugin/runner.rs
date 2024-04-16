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

use clap::Parser;
use env_logger::Builder;
use log::{Level, LevelFilter};
use nuts_backend::Backend;
use std::io::Write;

use crate::bson::BsonError;
use crate::plugin::cli::{PluginCli, PluginCommand};
use crate::plugin::handler::{InfoHandler, OpenCreateHandler, PluginHandler};

/// Runs the plugin.
///
/// The `PluginRunner` utility brings [`PluginHandler`] and
/// [cli](crate::plugin::cli) together and starts the plugin process.
///
/// You need to configure the [logging](`Self::configure_logging`) and finally
/// call [`Self::run`] to start the process.
pub struct PluginRunner<B: Backend, T: PluginHandler<B>> {
    cli: PluginCli<T::CreateArgs>,
    handler: T,
}

impl<B: Backend, T: PluginHandler<B>> PluginRunner<B, T> {
    /// Creates a new `PluginRunner` instance.
    ///
    /// The given [`PluginHandler`] is attached to the runner.
    pub fn new(handler: T) -> PluginRunner<B, T> {
        PluginRunner {
            cli: PluginCli::parse(),
            handler,
        }
    }

    /// Configures logging for the plugin process. You need to call this method
    /// before [`Self::run()`] in order to have some logging.
    pub fn configure_logging(&mut self) -> &mut Self {
        let mut builder = Builder::new();

        builder.filter_level(match self.cli.verbose {
            0 => LevelFilter::Off,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        });
        builder.format(|buf, record| {
            let prefix = match record.metadata().level() {
                Level::Error => "nuts-log-error",
                Level::Warn => "nuts-log-warn",
                Level::Info => "nuts-log-info",
                Level::Debug => "nuts-log-debug",
                Level::Trace => "nuts-log-trace",
            };

            writeln!(buf, "{}: {}", prefix, record.args())
        });
        builder.init();

        self
    }

    /// Runs the plugin process.
    pub fn run(self) -> Result<(), BsonError> {
        match self.cli.command {
            PluginCommand::Info(args) => InfoHandler::new(self.handler).run(&args),
            _ => OpenCreateHandler::new(&self.cli.command, self.handler).run(),
        }
    }
}
