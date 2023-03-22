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

use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    Open,
    Closed,
    NoSuchOption(String),
    InvalidOption(String),
    Backend(Box<dyn error::Error + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Open => write!(fmt, "The backend is already open"),
            Error::Closed => write!(fmt, "The backend is not open"),
            Error::NoSuchOption(option) => write!(fmt, "No such option: {}", option),
            Error::InvalidOption(option) => write!(fmt, "Invalid option: {}", option),
            Error::Backend(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
