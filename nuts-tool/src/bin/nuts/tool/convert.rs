// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use nuts::container::{Cipher, Digest};
use nutsbackend_directory::DirectoryId;

pub trait Convert
where
    Self: Sized,
{
    fn from_str(s: &str) -> Result<Self, String>;
    fn to_str(&self) -> String;
}

impl Convert for DirectoryId {
    fn from_str(s: &str) -> Result<Self, String> {
        s.parse::<DirectoryId>()
            .map_err(|_| format!("invalid id: {}", s))
    }

    fn to_str(&self) -> String {
        self.to_string()
    }
}

impl Convert for Cipher {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.as_ref() {
            "none" => Ok(Cipher::None),
            "aes128-ctr" => Ok(Cipher::Aes128Ctr),
            _ => Err(format!("invalid cipher: {}", s)),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Cipher::None => "none",
            Cipher::Aes128Ctr => "aes128-ctr",
        }
        .to_string()
    }
}

impl Convert for Digest {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "sha1" => Ok(Digest::Sha1),
            _ => Err(format!("invalid digest: {}", s)),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Digest::Sha1 => String::from("sha1"),
        }
    }
}