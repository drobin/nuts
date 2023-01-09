// MIT License
//
// Copyright (c) 2022 Robin Doer
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
use clap::{App, ArgMatches};

use crate::tool::actions::{container_arg, open_container};
use crate::tool::convert::Convert;
use crate::tool::kdf::KdfSpec;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Prints general information about the container.")
        .arg(container_arg())
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let container = open_container(args)?;
    let info = container.info()?;

    println!("block size: {}", info.backend.bsize);
    println!("cipher:     {}", info.cipher.to_str());

    match info.kdf {
        Some(kdf) => {
            let spec: KdfSpec = kdf.into();
            println!("kdf:        {}", spec.to_str());
        }
        None => println!("kdf:        none"),
    };

    Ok(())
}
