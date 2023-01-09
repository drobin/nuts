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

use std::io::{self, Write};
use std::result::Result;

use crate::tool::convert::Convert;
use crate::tool::hex::HexWriter;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Raw,
    Hex,
}

impl Convert for Format {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "raw" => Ok(Format::Raw),
            "hex" => Ok(Format::Hex),
            _ => Err(format!("invalid format: {}", s)),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Format::Raw => String::from("raw"),
            Format::Hex => String::from("hex"),
        }
    }
}

#[derive(Debug)]
pub struct Output(Option<HexWriter>);

impl Output {
    pub fn new(format: Format) -> Output {
        match format {
            Format::Raw => Output(None),
            Format::Hex => Output(Some(HexWriter::new())),
        }
    }

    pub fn print<T: AsRef<[u8]>>(&mut self, buf: T) {
        match self.0 {
            Some(ref mut hex) => {
                hex.fill(buf);
                hex.print();
            }
            None => io::stdout().write_all(&buf.as_ref()).unwrap(),
        }
    }

    pub fn flush(&mut self) {
        match self.0 {
            Some(ref mut hex) => hex.flush(),
            None => io::stdout().flush().unwrap(),
        }
    }
}
