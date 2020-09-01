// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use clap::ArgMatches;

use nuts::container::Container;

use crate::tool;

pub fn run(sub: &ArgMatches) -> tool::result::Result<()> {
    tool::logger::update(sub);

    let path = sub.value_of("PATH").unwrap();
    let mut container = Container::new();

    container.set_password_callback(tool::utils::ask_for_password);
    container.open(path, None)?;

    let digest = container
        .digest()?
        .map_or_else(|| String::from("none"), |d| d.to_string());

    say!(sub, "cipher:           {}", container.cipher()?);
    say!(sub, "digest:           {}", digest);
    say!(sub, "disk type:        {}", container.dtype()?);
    say!(sub, "block size:       {}", container.bsize()?);
    say!(sub, "blocks:           {}", container.blocks()?);
    say!(sub, "allocated blocks: {}", container.ablocks()?);

    Ok(())
}
