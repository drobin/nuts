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

use lazy_static::lazy_static;
use rpassword::prompt_password;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::fd::{FromRawFd, RawFd};
use std::path::PathBuf;

use crate::cli::global::{PasswordSource as LegacyPasswordSource, GLOBALS};

#[derive(Clone, Debug)]
pub enum PasswordSource {
    Fd(RawFd),
    Path(PathBuf),
    Console,
}

impl PasswordSource {
    pub fn new(fd: Option<RawFd>, path: Option<PathBuf>) -> PasswordSource {
        if let Some(fd) = fd {
            Self::Fd(fd)
        } else if let Some(path) = path {
            Self::Path(path)
        } else {
            Self::Console
        }
    }
}

impl Default for PasswordSource {
    fn default() -> Self {
        Self::Console
    }
}

fn ask_for_password() -> Result<Vec<u8>, String> {
    lazy_static! {
        static ref RESULT: Result<String, String> =
            prompt_password("Enter a password: ").map_err(|err| err.to_string());
    }

    match RESULT.as_ref() {
        Ok(s) => Ok(s.as_bytes().to_vec()),
        Err(s) => Err(s.clone()),
    }
}

fn ask_for_password_twice(prompt: &str) -> Result<Vec<u8>, String> {
    let pass1 = prompt_password(format!("{}: ", prompt)).map_err(|err| err.to_string())?;
    let pass2 = prompt_password(format!("{} (repeat): ", prompt)).map_err(|err| err.to_string())?;

    if pass1 == pass2 {
        Ok(pass1.as_bytes().to_vec())
    } else {
        Err("The passwords do not match".to_string())
    }
}

fn password_from_file(file: File) -> io::Result<Vec<u8>> {
    let mut reader = BufReader::new(file);
    let mut buf = vec![];

    reader.read_until(0x0a, &mut buf)?;

    if let Some(n) = buf.last() {
        if *n == 0x0a {
            buf.pop();
        }
    }

    Ok(buf)
}

fn password_from_source_or<F: FnOnce() -> Result<Vec<u8>, String>>(
    source: &LegacyPasswordSource,
    f: F,
) -> Result<Vec<u8>, String> {
    let file = match source {
        LegacyPasswordSource::Fd(fd) => unsafe { Some(Ok(File::from_raw_fd(*fd))) },
        LegacyPasswordSource::Path(path) => Some(File::open(path)),
        LegacyPasswordSource::Console => None,
    };

    match file {
        Some(Ok(f)) => password_from_file(f).map_err(|err| err.to_string()),
        Some(Err(err)) => Err(err.to_string()),
        None => f(),
    }
}

pub fn password_from_source() -> Result<Vec<u8>, String> {
    GLOBALS.with_borrow(|g| password_from_source_or(&g.password_source, ask_for_password))
}

pub fn password_from_source_twice(
    source: &LegacyPasswordSource,
    prompt: &str,
) -> Result<Vec<u8>, String> {
    password_from_source_or(source, || ask_for_password_twice(prompt))
}

fn new_password_from_source_or<F: FnOnce() -> Result<Vec<u8>, String>>(
    source: PasswordSource,
    f: F,
) -> Result<Vec<u8>, String> {
    let file = match source {
        PasswordSource::Fd(fd) => unsafe { Some(Ok(File::from_raw_fd(fd))) },
        PasswordSource::Path(path) => Some(File::open(path)),
        PasswordSource::Console => None,
    };

    match file {
        Some(Ok(f)) => password_from_file(f).map_err(|err| err.to_string()),
        Some(Err(err)) => Err(err.to_string()),
        None => f(),
    }
}

pub fn new_password_from_source(source: PasswordSource) -> Result<Vec<u8>, String> {
    new_password_from_source_or(source, ask_for_password)
}

pub fn new_password_from_source_twice(
    source: PasswordSource,
    prompt: &str,
) -> Result<Vec<u8>, String> {
    new_password_from_source_or(source, || ask_for_password_twice(prompt))
}
