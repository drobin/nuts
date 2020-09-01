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

pub enum Format {
    String,
    Hex,
}

impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::String => String::from("string"),
            Format::Hex => String::from("hex"),
        }
    }

    pub fn from_string(s: &str) -> Result<Format, String> {
        match s {
            "string" => Ok(Format::String),
            "hex" => Ok(Format::Hex),
            _ => Err(format!("invalid format: {}", s)),
        }
    }

    pub fn default() -> Format {
        Format::String
    }

    pub fn validate(s: String) -> Result<(), String> {
        Format::from_string(&s).map(|_| ())
    }
}
