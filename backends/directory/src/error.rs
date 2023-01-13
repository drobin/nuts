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

use std::str::FromStr;
use std::{error, fmt, io, result};

use uuid::Uuid;

#[derive(Debug)]
pub enum DirectoryError {
    Io(io::Error),
    Exists,
    UniqueId,
    InvalidId(<Uuid as FromStr>::Err),
    InvalidBlockSize(u32),
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DirectoryError::Io(cause) => fmt::Display::fmt(cause, fmt),
            DirectoryError::Exists => write!(fmt, "The container already exists"),
            DirectoryError::UniqueId => write!(fmt, "could not generate a unique id"),
            DirectoryError::InvalidId(cause) => write!(fmt, "The id is invalid: {}", cause),
            DirectoryError::InvalidBlockSize(n) => write!(fmt, "The block-size is invalid: {}", n),
        }
    }
}

impl error::Error for DirectoryError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DirectoryError::Io(cause) => Some(cause),
            DirectoryError::Exists | DirectoryError::UniqueId => None,
            DirectoryError::InvalidId(cause) => Some(cause),
            DirectoryError::InvalidBlockSize(_) => None,
        }
    }
}

impl From<io::Error> for DirectoryError {
    fn from(cause: io::Error) -> Self {
        DirectoryError::Io(cause)
    }
}

pub type DirectoryResult<T> = result::Result<T, DirectoryError>;
