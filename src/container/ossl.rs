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

pub(super) mod error;
pub(super) mod evp;
pub(super) mod rand;

use std::os::raw::c_int;
use std::ptr;

use crate::container::ossl::error::OpenSSLResult;

pub use error::OpenSSLError;

trait MapResult
where
    Self: PartialOrd<c_int> + Sized,
{
    fn into_result(self) -> OpenSSLResult<Self> {
        self.map_result(|n| n)
    }

    fn map_result<F, T>(self, op: F) -> OpenSSLResult<T>
    where
        F: FnOnce(Self) -> T,
    {
        if self > 0 {
            Ok(op(self))
        } else {
            Err(OpenSSLError::library())
        }
    }
}

trait MapResultPtr<T>
where
    Self: PartialEq<*mut T> + Sized,
{
    fn into_result(self) -> OpenSSLResult<Self> {
        self.map_result(|n| n)
    }

    fn map_result<F, R>(self, op: F) -> OpenSSLResult<R>
    where
        F: FnOnce(Self) -> R,
    {
        if self == ptr::null_mut() {
            Err(OpenSSLError::library())
        } else {
            Ok(op(self))
        }
    }
}

impl MapResult for c_int {}
impl<T> MapResultPtr<T> for *mut T {}
