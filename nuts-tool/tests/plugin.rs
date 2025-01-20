// MIT License
//
// Copyright (c) 2024,2025 Robin Doer
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

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
mod predicates_ext;

use assert_cmd::Command;
use assert_fs::fixture::TempDir;
use clap::crate_version;
use predicates::prelude::PredicateBooleanExt;
use predicates::str;
use std::fs;
use std::path::Path;

use crate::common::{nuts_tool, plugin_add, plugin_path, plugin_remove};
use crate::predicates_ext::{hash, list};

fn plugin_modify(home: &Path, name: &str) -> Command {
    nuts_tool(home, ["plugin", "modify", name])
}

fn plugin_list(home: &Path) -> Command {
    nuts_tool(home, ["plugin", "list"])
}

fn plugin_info(home: &Path, name: &str) -> Command {
    nuts_tool(home, ["plugin", "info", name])
}

#[test]
fn help() {
    let tmp_dir = TempDir::new().unwrap();

    for args in [
        ["plugin", "help"].as_slice(),
        ["plugin", "add", "--help"].as_slice(),
        ["plugin", "modify", "--help"].as_slice(),
        ["plugin", "remove", "--help"].as_slice(),
        ["plugin", "info", "--help"].as_slice(),
        ["plugin", "list", "--help"].as_slice(),
    ] {
        let no_password_from_fd = predicates::str::contains("--password-from-fd").not();
        let no_password_from_file = predicates::str::contains("--password-from-file").not();
        let no_container = predicates::str::contains("--container").not();
        let verbose = predicates::str::contains("--verbose");
        let quiet = predicates::str::contains("--quiet");

        nuts_tool(&tmp_dir, args)
            .assert()
            .success()
            .stdout(
                no_password_from_fd
                    .and(no_password_from_file)
                    .and(no_container)
                    .and(verbose)
                    .and(quiet),
            )
            .stderr("");
    }
}

#[test]
fn add() {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");

    plugin_add(&tmp_dir, "directory", "no_such_file")
        .assert()
        .code(1)
        .stdout("")
        .stderr("the path 'no_such_file' is invalid\n");
    plugin_list(&tmp_dir).assert().success().stdout("");

    plugin_add(&tmp_dir, "directory", &plugin.to_string_lossy())
        .assert()
        .success()
        .stdout("")
        .stderr("");
    plugin_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["directory"]));
    plugin_info(&tmp_dir, "directory").assert().success();

    plugin_add(&tmp_dir, "directory", &plugin.to_string_lossy())
        .assert()
        .code(1)
        .stdout("")
        .stderr("the plugin 'directory' is already configured\n");
}

#[test]
fn modify() {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");
    let new_plugin = tmp_dir.path().join("new_plugin");

    fs::copy(&plugin, &new_plugin).unwrap();

    plugin_modify(&tmp_dir, "directory")
        .args(["--path", &plugin.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout("")
        .stderr("the plugin 'directory' is not configured\n");
    plugin_list(&tmp_dir).assert().success().stdout("");

    plugin_add(&tmp_dir, "directory", plugin.to_str().unwrap())
        .assert()
        .success();

    plugin_modify(&tmp_dir, "directory")
        .args(["--path", "no_such_file"])
        .assert()
        .code(1)
        .stdout("")
        .stderr("the path \'no_such_file\' is invalid\n");
    plugin_info(&tmp_dir, "directory")
        .assert()
        .success()
        .stdout(hash::eq([
            ("name", "directory"),
            ("revision", "1"),
            ("version", crate_version!()),
            ("path", plugin.to_str().unwrap()),
        ]));

    plugin_modify(&tmp_dir, "directory")
        .args(["--path", new_plugin.to_str().unwrap()])
        .assert()
        .success()
        .stdout("")
        .stderr("");
    plugin_info(&tmp_dir, "directory")
        .assert()
        .success()
        .stdout(hash::eq([
            ("name", "directory"),
            ("revision", "1"),
            ("version", crate_version!()),
            ("path", new_plugin.to_str().unwrap()),
        ]));
}

#[test]
fn remove() {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");

    plugin_remove(&tmp_dir, "directory")
        .assert()
        .code(1)
        .stdout("")
        .stderr("the plugin 'directory' is not configured\n");

    plugin_add(&tmp_dir, "directory", &plugin.to_string_lossy())
        .assert()
        .success();

    plugin_remove(&tmp_dir, "directory")
        .assert()
        .success()
        .stdout("")
        .stderr("");

    plugin_list(&tmp_dir).assert().success().stdout("");
}

#[test]
fn info() {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");

    plugin_info(&tmp_dir, "directory")
        .assert()
        .code(1)
        .stdout("")
        .stderr("the plugin 'directory' is not configured\n");

    plugin_add(&tmp_dir, "directory", &plugin.to_string_lossy())
        .assert()
        .success();

    plugin_info(&tmp_dir, "directory")
        .assert()
        .success()
        .stdout(hash::eq([
            ("name", "directory"),
            ("revision", "1"),
            ("version", crate_version!()),
            ("path", plugin.to_str().unwrap()),
        ]));
}

#[test]
fn list() {
    let tmp_dir = TempDir::new().unwrap();
    let plugin = plugin_path("nuts-directory");

    plugin_list(&tmp_dir)
        .assert()
        .success()
        .stdout("")
        .stderr("");

    plugin_add(&tmp_dir, "plugin1", &plugin.to_string_lossy())
        .assert()
        .success();
    plugin_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["plugin1"]))
        .stderr("");

    plugin_add(&tmp_dir, "plugin2", &plugin.to_string_lossy())
        .assert()
        .success();
    plugin_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["plugin1", "plugin2"]))
        .stderr("");
}
