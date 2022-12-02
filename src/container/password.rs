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

use std::rc::Rc;
use std::{fmt, result};

use crate::backend::Backend;
use crate::container::error::{ContainerError, ContainerResult};
use crate::whiteout_vec;

pub struct PasswordStore {
    callback: Option<Rc<dyn Fn() -> result::Result<Vec<u8>, String>>>,
    value: Option<Vec<u8>>,
}

impl PasswordStore {
    pub fn new(callback: Option<Rc<dyn Fn() -> result::Result<Vec<u8>, String>>>) -> PasswordStore {
        PasswordStore {
            callback,
            value: None,
        }
    }

    pub fn value<B: Backend>(&mut self) -> ContainerResult<&[u8], B> {
        match self.value {
            Some(ref v) => Ok(v),
            None => {
                let callback = self
                    .callback
                    .as_ref()
                    .ok_or_else(|| ContainerError::NoPassword(None))?;
                let value = callback().map_err(|cause| ContainerError::NoPassword(Some(cause)))?;

                self.value = Some(value);

                Ok(self.value.as_ref().unwrap())
            }
        }
    }
}

impl Drop for PasswordStore {
    fn drop(&mut self) {
        match &mut self.value {
            Some(vec) => whiteout_vec(vec),
            None => {}
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
