// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

#[cfg(test)]
mod tests;

use std::fmt;
use std::rc::Rc;
use thiserror::Error;

use crate::svec::SecureVec;

/// Password related error codes.
#[derive(Debug, Error)]
pub enum PasswordError {
    /// No password callback is assigned to the container, thus no password
    /// is available.
    #[error("a password is needed by the current cipher")]
    NoPassword,

    /// The password callback generated an error, which is passed to the
    /// variant.
    #[error("failed to receive the password: {0}")]
    PasswordCallback(String),
}

pub struct PasswordStore {
    callback: Option<Rc<dyn Fn() -> Result<Vec<u8>, String>>>,
    value: Option<SecureVec>,
}

impl PasswordStore {
    pub fn new(callback: Option<Rc<dyn Fn() -> Result<Vec<u8>, String>>>) -> PasswordStore {
        PasswordStore {
            callback,
            value: None,
        }
    }

    #[cfg(test)]
    pub fn with_value(value: &[u8]) -> PasswordStore {
        PasswordStore {
            callback: None,
            value: Some(value.to_vec().into()),
        }
    }

    pub fn value(&mut self) -> Result<&[u8], PasswordError> {
        match self.value {
            Some(ref v) => Ok(v),
            None => {
                let callback = self
                    .callback
                    .as_ref()
                    .ok_or_else(|| PasswordError::NoPassword)?;
                let value = callback().map_err(|cause| PasswordError::PasswordCallback(cause))?;

                self.value = Some(value.into());

                Ok(self.value.as_ref().unwrap())
            }
        }
    }
}

impl fmt::Debug for PasswordStore {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let callback = match self.callback {
            Some(_) => Some(()),
            None => None,
        };

        let value = match self.value {
            Some(_) => Some("***"),
            None => None,
        };

        fmt.debug_struct("PasswordStore")
            .field("callback", &callback)
            .field("value", &value)
            .finish()
    }
}
