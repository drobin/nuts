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

use nuts::container::{Digest, Kdf};

use crate::tool::convert::Convert;

#[derive(Debug)]
pub struct KdfSpec {
    pub algorithm: String,
    pub digest: Option<Digest>,
    pub iterations: Option<u32>,
    pub salt_len: Option<u32>,
}

impl KdfSpec {
    fn new(algorithm: &str) -> KdfSpec {
        KdfSpec {
            algorithm: algorithm.to_string(),
            digest: None,
            iterations: None,
            salt_len: None,
        }
    }

    fn parse(s: &str) -> Result<Self, String> {
        let v: Vec<&str> = s
            .split(':')
            .map(|s| s.trim_matches(char::is_whitespace))
            .collect();

        if v.is_empty() {
            return Err(String::from("empty KDF specification"));
        }

        match v[0] {
            "pbkdf2" => Self::parse_pbkdf2(&v),
            _ => Err(format!("unknown KDF: {}", v[0])),
        }
    }

    fn parse_pbkdf2(v: &[&str]) -> Result<KdfSpec, String> {
        if v.len() == 1 {
            Ok(KdfSpec::new(v[0]))
        } else if v.len() == 4 {
            let mut spec = KdfSpec::new(v[0]);

            if !v[1].is_empty() {
                spec.digest = Some(Digest::from_str(v[1])?);
            }

            if !v[2].is_empty() {
                spec.iterations = Some(Self::parse_u32(v[2])?);
            }

            if !v[3].is_empty() {
                spec.salt_len = Some(Self::parse_u32(v[3])?);
            }

            Ok(spec)
        } else {
            Err(String::from("invalid KDF specification"))
        }
    }

    fn parse_u32(s: &str) -> Result<u32, String> {
        s.parse::<u32>().map_err(|e| e.to_string())
    }
}

impl From<Kdf> for KdfSpec {
    fn from(kdf: Kdf) -> Self {
        let Kdf::Pbkdf2 {
            digest,
            iterations,
            salt,
        } = kdf;

        KdfSpec {
            algorithm: "pbkdf2".to_string(),
            digest: Some(digest),
            iterations: Some(iterations),
            salt_len: Some(salt.len() as u32),
        }
    }
}

impl Convert for KdfSpec {
    fn from_str(s: &str) -> Result<Self, String> {
        KdfSpec::parse(s)
    }

    fn to_str(&self) -> String {
        let digest = match self.digest {
            Some(d) => d.to_str(),
            None => "".to_string(),
        };
        let iterations = match self.iterations {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let salt_len = match self.salt_len {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };

        format!("{}:{}:{}:{}", self.algorithm, digest, iterations, salt_len)
    }
}
