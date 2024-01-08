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

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local, Utc};
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum TimeFormat {
    Local,
    Utc,
}

impl TimeFormat {
    pub fn format<'a>(&self, dt: &DateTime<Utc>, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        match self {
            Self::Local => Into::<DateTime<Local>>::into(*dt).format(fmt),
            Self::Utc => dt.format(fmt),
        }
    }
}
