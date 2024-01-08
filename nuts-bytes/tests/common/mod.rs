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

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SampleError(pub String);

macro_rules! assert_sample_error {
    ($err:expr) => {
        match $err {
            nuts_bytes::Error::Custom(b) => {
                if let Ok(cause) = b.downcast::<crate::common::SampleError>() {
                    assert_eq!(cause.0, "sample error");
                } else {
                    panic!("Not a SampleError");
                }
            }
            _ => panic!("Invalid error: {:?}", $err),
        }
    };
}

pub(crate) use assert_sample_error;

#[allow(dead_code)]
pub mod map_eq_mod {
    use crate::common::SampleError;

    pub fn from_bytes<T>(n: T) -> Result<T, SampleError> {
        Ok(n)
    }

    pub fn to_bytes<T: Copy>(n: &T) -> Result<T, SampleError> {
        Ok(*n)
    }
}

#[allow(dead_code)]
pub mod map_mod {
    use crate::common::SampleError;

    pub fn from_bytes(n: u8) -> Result<u16, SampleError> {
        Ok(n as u16 + 1)
    }

    pub fn to_bytes(n: &u8) -> Result<u16, SampleError> {
        Ok(*n as u16 + 1)
    }
}

#[allow(dead_code)]
pub fn map_from_bytes(n: u8) -> Result<u16, SampleError> {
    Ok(n as u16 + 2)
}

#[allow(dead_code)]
pub fn map_to_bytes(n: &u8) -> Result<u16, SampleError> {
    Ok(*n as u16 + 2)
}

#[allow(dead_code)]
pub fn map_err_from_bytes<T>(_n: T) -> Result<T, SampleError> {
    Err(SampleError("sample error".to_string()))
}

#[allow(dead_code)]
pub fn map_err_to_bytes<T>(_n: &T) -> Result<T, SampleError> {
    Err(SampleError("sample error".to_string()))
}

#[allow(dead_code)]
pub mod map_err_mod {
    use crate::common::SampleError;

    pub fn from_bytes<T>(_n: T) -> Result<T, SampleError> {
        Err(SampleError("sample error".to_string()))
    }

    pub fn to_bytes<T>(_n: &T) -> Result<T, SampleError> {
        Err(SampleError("sample error".to_string()))
    }
}
