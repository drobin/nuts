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

use assert_cmd::Command;
use assert_fs::TempDir;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str;

pub fn plugin_path(name: &str) -> PathBuf {
    Command::cargo_bin(name).unwrap().get_program().into()
}

pub fn nuts_tool<I, S>(home: &Path, args: I) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::cargo_bin("nuts").unwrap();

    cmd.env("HOME", home).args(args);

    cmd
}

pub fn plugin_add(home: &Path, name: &str, plugin: &str) -> Command {
    nuts_tool(home, ["plugin", "add", "--path", plugin, name])
}

pub fn plugin_remove(home: &Path, name: &str) -> Command {
    nuts_tool(home, ["plugin", "remove", name])
}

pub fn container_create(home: &Path, name: &str, plugin: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "create", "--plugin", plugin, name]);

    handle_password_args(cmd, pass)
}

pub fn handle_password_args(mut cmd: Command, pass: Option<&[u8]>) -> Command {
    if let Some(bytes) = pass {
        cmd.args(["--password-from-fd", "0"]).write_stdin(bytes);
    }

    cmd
}

pub fn handle_password_file(
    home: &Path,
    mut cmd: Command,
    arg: &str,
    pass: Option<&[u8]>,
) -> Command {
    if let Some(bytes) = pass {
        let path = home.join("password.txt");

        let mut f = File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        f.flush().unwrap();

        cmd.args([arg, path.to_str().unwrap()]);
    }

    cmd
}

pub fn setup() -> TempDir {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");

    plugin_add(&tmp_dir, "directory", plugin.to_str().unwrap())
        .assert()
        .success();

    tmp_dir
}
