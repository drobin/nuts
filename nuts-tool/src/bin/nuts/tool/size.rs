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

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct Size<T>(T);

impl<T> Deref for Size<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: fmt::Display> fmt::Display for Size<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<T: TryFrom<u64>> FromStr for Size<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
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

        match s.parse::<u64>() {
            Ok(size) => match T::try_from(factor * size) {
                Ok(nbytes) => Ok(Size(nbytes)),
                Err(_) => Err(format!("size {} is too large", size)),
            },
            Err(cause) => Err(format!("{}", cause)),
        }
    }
}
