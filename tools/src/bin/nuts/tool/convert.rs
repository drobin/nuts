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
use regex::Regex;
use std::convert::TryFrom;

use crate::tool::format::Format;
use crate::tool::id::IdRange;
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
            "raw" => Ok(Format::Raw),
            "string" => Ok(Format::String),
            "hex" => Ok(Format::Hex),
            _ => Err(Error::new(&format!("invalid format: {}", s))),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Format::Raw => String::from("raw"),
            Format::String => String::from("string"),
            Format::Hex => String::from("hex"),
        }
    }
}

impl Convert for IdRange {
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(^\d+$)|^(\d*)\.\.(\d*)$").unwrap();
        }

        let caps = RE.captures(s).ok_or_else(|| {
            let msg = format!("invalid range: {}", s);
            Error::new(&msg)
        })?;

        let (start, end) = if caps.get(1).is_some() {
            // matches a single number

            assert!(caps.get(2).is_none(), "invalid capture#2 for '{}'", s);
            assert!(caps.get(3).is_none(), "invalid capture#3 for '{}'", s);

            let start = caps.get(1).unwrap().as_str().parse::<u64>()?;

            (Some(start), Some(start))
        } else {
            // matches a range: n..m/n../..m/..

            assert!(caps.get(1).is_none(), "invalid capture#1 for '{}'", s);

            let start = caps.get(2).unwrap().as_str();
            let end = caps.get(3).unwrap().as_str();

            let start = match start {
                "" => None,
                _ => Some(start.parse::<u64>()?),
            };

            let end = match end {
                "" => None,
                _ => Some(end.parse::<u64>()?),
            };

            (start, end)
        };

        Ok(IdRange::new(start, end))
    }

    fn to_str(&self) -> String {
        let start = self.start().map_or("".to_string(), |s| s.to_string());
        let end = self.end().map_or("".to_string(), |e| e.to_string());

        format!("{}..{}", start, end)
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

pub struct WrappingKeySpec {
    pub algorithm: String,
    pub iterations: Option<u32>,
    pub salt_len: Option<u32>,
}

impl WrappingKeySpec {
    fn new(algorithm: &str) -> WrappingKeySpec {
        WrappingKeySpec {
            algorithm: algorithm.to_string(),
            iterations: None,
            salt_len: None,
        }
    }
}

fn from_pbkdf2_str(v: &Vec<&str>) -> Result<WrappingKeySpec> {
    if v.len() == 1 {
        Ok(WrappingKeySpec::new(v[0]))
    } else if v.len() == 3 {
        let mut spec = WrappingKeySpec::new(v[0]);

        if !v[1].is_empty() {
            spec.iterations = Some(v[1].parse::<u32>()?);
        }

        if !v[2].is_empty() {
            spec.salt_len = Some(v[2].parse::<u32>()?);
        }

        Ok(spec)
    } else {
        let msg = "invalid wrapping key specification";
        Err(Error::new(&msg))
    }
}

impl Convert for WrappingKeySpec {
    fn from_str(s: &str) -> Result<Self> {
        let v: Vec<&str> = s
            .split(':')
            .map(|s| s.trim_matches(char::is_whitespace))
            .collect();

        if v.is_empty() {
            let msg = "empty wrapping key specification";
            return Err(Error::new(&msg));
        }

        match v[0] {
            "pbkdf2" => from_pbkdf2_str(&v),
            _ => {
                let msg = format!("unknown wrapping key algorithm: {}", v[0]);
                Err(Error::new(&msg))
            }
        }
    }

    fn to_str(&self) -> String {
        let iterations = match self.iterations {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let salt_len = match self.salt_len {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };

        format!("{}:{}:{}", self.algorithm, iterations, salt_len)
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
