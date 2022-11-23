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

use std::ffi::CStr;
use std::os::raw::{c_char, c_ulong};
use std::{error, fmt, result};

extern "C" {
    fn ERR_get_error() -> c_ulong;
    fn ERR_lib_error_string(e: c_ulong) -> *const c_char;
    fn ERR_func_error_string(e: c_ulong) -> *const c_char;
    fn ERR_reason_error_string(e: c_ulong) -> *const c_char;
}

#[derive(Debug)]
pub enum OpenSSLError {
    Library(LibraryError),
    InvalidKey,
    InvalidIv,
    InvalidBlockSize,
}

impl OpenSSLError {
    pub(crate) fn library() -> OpenSSLError {
        OpenSSLError::Library(LibraryError::get())
    }
}

impl fmt::Display for OpenSSLError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenSSLError::Library(cause) => fmt::Display::fmt(cause, fmt),
            OpenSSLError::InvalidKey => write!(fmt, "Invalid key"),
            OpenSSLError::InvalidIv => write!(fmt, "Invalid iv"),
            OpenSSLError::InvalidBlockSize => write!(fmt, "Invalid block-size"),
        }
    }
}

impl error::Error for OpenSSLError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            OpenSSLError::Library(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Error type which collects error from the underlaying OpenSSL library.
#[derive(Debug)]
pub struct LibraryError(c_ulong);

impl LibraryError {
    /// Returns the earliest error from the thread's error queue and removes
    /// the entry. This function can be called repeatedly until there are no
    /// more errors to return.
    fn get() -> LibraryError {
        let error_code = unsafe { ERR_get_error() };
        LibraryError(error_code)
    }

    /// Returns the library name of the latest error.
    pub fn library(&self) -> String {
        let s = unsafe { CStr::from_ptr(ERR_lib_error_string(self.0)) };
        s.to_string_lossy().into_owned()
    }

    /// Returns the function name of the lastest error.
    pub fn function(&self) -> String {
        let s = unsafe { CStr::from_ptr(ERR_func_error_string(self.0)) };
        s.to_string_lossy().into_owned()
    }

    /// Returns a reason string of the latest error.
    pub fn reason(&self) -> String {
        let s = unsafe { CStr::from_ptr(ERR_reason_error_string(self.0)) };
        s.to_string_lossy().into_owned()
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}:{}:{}:{}",
            self.0,
            self.library(),
            self.function(),
            self.reason()
        )
    }
}

impl error::Error for LibraryError {}

pub type OpenSSLResult<T> = result::Result<T, OpenSSLError>;
