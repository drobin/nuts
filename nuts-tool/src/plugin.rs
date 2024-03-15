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

use anyhow::{anyhow, Result};
use log::Level::{Error, Trace};
use log::{debug, error, log_enabled, trace, warn};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

fn read_all<R: Read>(source: &mut Option<R>) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];

    match source.as_mut() {
        Some(r) => r.read_to_end(&mut buf).map(|_| buf),
        None => {
            warn!("source stream not available");
            Ok(buf)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

pub struct Plugin<'a>(&'a Path);

impl<'a> Plugin<'a> {
    pub fn new(binary: &Path) -> Plugin {
        Plugin(binary)
    }

    pub fn info(&self) -> Result<Info> {
        let out = self.execute(&["info", "--format=bson"])?;
        let info = bson::from_slice::<Info>(&out)?;

        if log_enabled!(Trace) {
            let fname = self.0.file_name().unwrap_or(OsStr::new("???"));
            trace!("{}: {:?}", fname.to_string_lossy(), info);
        }

        Ok(info)
    }

    fn execute(&self, args: &[&str]) -> Result<Vec<u8>> {
        const TIMEOUT: u64 = 2;
        debug!("executing {} with {:?}", self.0.display(), args);

        let mut child = Command::new(&self.0)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(status) = child.wait_timeout(Duration::from_secs(TIMEOUT))? {
            if status.success() {
                Ok(read_all(&mut child.stdout)?)
            } else {
                let stderr = read_all(&mut child.stderr)?;

                if log_enabled!(Error) && stderr.len() > 0 {
                    let mut reader = BufReader::new(Cursor::new(&stderr));
                    let mut line_buf = Vec::<u8>::new();

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
                }

                let fname = self.0.file_name().unwrap_or(OsStr::new("???"));

                Err(anyhow!(
                    "failed to fetch information from {}",
                    fname.to_string_lossy()
                ))
            }
        } else {
            Err(anyhow!("timeout {} secs elapsed", TIMEOUT))
        }
    }
}
