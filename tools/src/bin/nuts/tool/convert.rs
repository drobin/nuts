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

use nuts::types::{Cipher, Digest, DiskType};
use std::convert::TryFrom;

use crate::tool::format::Format;
use crate::tool::result::{Error, Result};

pub struct Size<T> {
    pub nbytes: T,
}

pub trait Convert
where
    Self: Sized,
{
    fn from_str(s: &str) -> Result<Self>;
    fn to_str(&self) -> String;
}

impl Convert for Cipher {
    fn from_str(s: &str) -> Result<Self> {
        match s.as_ref() {
            "none" => Ok(Cipher::None),
            "aes128-ctr" => Ok(Cipher::Aes128Ctr),
            _ => {
                let msg = format!("invalid cipher: {}", s);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        match self {
            Cipher::None => String::from("none"),
            Cipher::Aes128Ctr => String::from("aes128-ctr"),
        }
    }
}

impl Convert for Option<Digest> {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "none" => Ok(None),
            "sha1" => Ok(Some(Digest::Sha1)),
            _ => {
                let msg = format!("invalid digest: {}", s);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        match self {
            None => String::from("none"),
            Some(Digest::Sha1) => String::from("sha1"),
        }
    }
}

impl Convert for DiskType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fat-zero" => Ok(DiskType::FatZero),
            "fat-random" => Ok(DiskType::FatRandom),
            "thin-zero" => Ok(DiskType::ThinZero),
            "thin-random" => Ok(DiskType::ThinRandom),
            _ => {
                let msg = format!("invalid disk-type: {}", s);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        match self {
            DiskType::FatZero => String::from("fat-zero"),
            DiskType::FatRandom => String::from("fat-random"),
            DiskType::ThinZero => String::from("thin-zero"),
            DiskType::ThinRandom => String::from("thin-random"),
        }
    }
}

impl Convert for Format {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "string" => Ok(Format::String),
            "hex" => Ok(Format::Hex),
            _ => Err(Error::new(&format!("invalid format: {}", s))),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Format::String => String::from("string"),
            Format::Hex => String::from("hex"),
        }
    }
}

impl<T> Convert for Size<T>
where
    T: TryFrom<u64> + ToString,
{
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_matches(char::is_whitespace).to_lowercase();

        let (s, factor) = if s.ends_with("kb") {
            (&s[..s.len() - 2], 1024)
        } else if s.ends_with("mb") {
            (&s[..s.len() - 2], 1024 * 1024)
        } else if s.ends_with("gb") {
            (&s[..s.len() - 2], 1024 * 1024 * 1024)
        } else {
            (&s[..], 1)
        };

        let size = s.parse::<u64>()?;

        match T::try_from(factor * size) {
            Ok(nbytes) => Ok(Size { nbytes }),
            Err(_) => {
                let msg = format!("size {} is too large", size);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        self.nbytes.to_string()
    }
}

impl Convert for u32 {
    fn from_str(s: &str) -> Result<Self> {
        let num: u64 = Convert::from_str(s)?;

        match Self::try_from(num) {
            Ok(n) => Ok(n),
            Err(_) => {
                let msg = format!("number {} is too large", num);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        "".to_string()
    }
}

impl Convert for u64 {
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_matches(char::is_whitespace).to_lowercase();
        Ok(s.parse::<u64>()?)
    }

    fn to_str(&self) -> String {
        self.to_string()
    }
}
