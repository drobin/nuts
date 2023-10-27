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

use cargo_metadata::Metadata;
use std::env;
use std::path::PathBuf;

const SHORT_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGES: [&str; 3] = ["nuts-archive", "nuts-container", "nuts-directory"];

fn cargo_metadata() -> Metadata {
    let cargo = env::var("CARGO").unwrap();
    let manifest_path: PathBuf = [
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        "Cargo.toml".to_string(),
    ]
    .iter()
    .collect();

    println!("CARGO: {}", cargo);
    println!("CARGO_MANIFEST_DIR: {}", manifest_path.display());

    let mut cmd = cargo_metadata::MetadataCommand::new();

    cmd.cargo_path(cargo);
    cmd.manifest_path(manifest_path);

    cmd.exec().unwrap()
}

fn main() {
    let metadata = cargo_metadata();

    let packages = metadata
        .packages
        .iter()
        .filter(|pkg| PACKAGES.iter().position(|name| name == &pkg.name).is_some());
    let version_str = packages
        .map(|pkg| format!("{}: {}", pkg.name, pkg.version))
        .collect::<Vec<String>>()
        .join(", ");

    let short_version = format!("{}", SHORT_VERSION);
    let long_version = format!("{} ({})", SHORT_VERSION, version_str);

    println!("cargo:rustc-env=NUTS_TOOL_SHORT_VERSION={}", short_version);
    println!("cargo:rustc-env=NUTS_TOOL_LONG_VERSION={}", long_version);
}
