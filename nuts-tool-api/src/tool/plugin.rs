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

use log::{debug, error, log_enabled, Level};
use std::io::{self, BufRead, BufReader, Cursor};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::{cmp, env};

use crate::bson::BsonReader;
use crate::tool::connection::PluginConnection;
use crate::tool::{PluginError, PluginResult};
use crate::PluginInfo;

fn log_as_error(bytes: Vec<u8>) -> io::Result<()> {
    let mut reader = BufReader::new(Cursor::new(bytes));
    let mut line_buf = vec![];

    loop {
        line_buf.clear();

        let n = reader.read_until(0x0A, &mut line_buf)?;

        if n > 0 {
            let line = String::from_utf8_lossy(&line_buf);
            error!("{}", line.trim());
        } else {
            break;
        }
    }

    Ok(())
}

pub struct Plugin<'a>(&'a Path);

impl<'a> Plugin<'a> {
    pub fn new(binary: &Path) -> Plugin {
        Plugin(binary)
    }

    pub fn open(&self, name: &str, verbose: u8) -> PluginResult<PluginConnection> {
        let child = self.new_child(Stdio::piped(), &Self::make_args("open", name, verbose, &[]))?;

        Ok(PluginConnection::new(child))
    }

    pub fn create(
        &self,
        name: &str,
        verbose: u8,
        extra_args: &[String],
    ) -> PluginResult<PluginConnection> {
        let child = self.new_child(
            Stdio::piped(),
            &Self::make_args("create", name, verbose, extra_args),
        )?;

        Ok(PluginConnection::new(child))
    }

    pub fn info(&self) -> PluginResult<PluginInfo> {
        let child = self.new_child(Stdio::null(), &["info", "--format=bson"])?;
        let output = child.wait_with_output()?;

        if output.status.success() {
            let mut reader = BsonReader::new(Cursor::new(output.stdout));

            match reader.read()? {
                Some(info) => {
                    debug!("plugin infos: {:?}", info);
                    Ok(info)
                }
                None => Err(PluginError::ShortPluginInfo),
            }
        } else {
            if log_enabled!(Level::Error) {
                log_as_error(output.stderr)?;
            }

            Err(PluginError::BadPluginInfo(output.status))
        }
    }

    fn make_args<'b>(
        command: &'b str,
        name: &'b str,
        verbose: u8,
        extra_args: &'b [String],
    ) -> Vec<&'b str> {
        let mut args = vec![command, name];

        extra_args
            .iter()
            .map(|s| s.as_str())
            .for_each(|s| args.push(s));

        let max = cmp::min(verbose, 3);
        (0..max).for_each(|_| args.push("-v"));

        args
    }

    fn new_child(&self, stdin: Stdio, args: &[&str]) -> PluginResult<Child> {
        debug!("executing {} with {:?}", self.0.display(), args);

        let mut command = Command::new(self.0);

        command
            .args(args)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(value) = env::var_os("RUST_BACKTRACE") {
            command.env("RUST_BACKTRACE", value);
        }

        Ok(command.spawn()?)
    }
}
