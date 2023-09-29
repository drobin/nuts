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

use std::env;
use std::io::{self, Write};
use std::process::Command;

const SHORT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn nuts_container_version() -> String {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out = Command::new("cargo")
        .current_dir(path)
        .arg("pkgid")
        .arg("nuts-container")
        .output()
        .unwrap();

    if !out.status.success() {
        io::stderr().write_all(&out.stderr).unwrap();
        panic!("failed to execute `cargo pkgid`");
    }

    let pkgid = String::from_utf8(out.stdout).unwrap();
    let splitted = pkgid.rsplitn(2, '#').collect::<Vec<&str>>();

    if splitted.len() != 2 {
        panic!("invalid pkgid: {}", pkgid.trim())
    }

    splitted[0].trim().to_string()
}

fn main() {
    let container_version = nuts_container_version();

    let short_version = format!("{}", SHORT_VERSION);
    let long_version = format!("{} (container: {})", SHORT_VERSION, container_version);

    println!("cargo:rustc-env=NUTS_TOOL_SHORT_VERSION={}", short_version);
    println!("cargo:rustc-env=NUTS_TOOL_LONG_VERSION={}", long_version);
}
