// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use crate::error::Error;
use crate::result::Result;
use crate::utils::SecureVec;

pub struct PasswordStore {
    callback: Option<Box<dyn Fn() -> Result<Vec<u8>>>>,
    value: Option<SecureVec<u8>>,
}

impl PasswordStore {
    pub fn new() -> PasswordStore {
        PasswordStore {
            callback: None,
            value: None,
        }
    }

    pub fn set_callback(&mut self, callback: impl Fn() -> Result<Vec<u8>> + 'static) {
        self.callback = Some(Box::new(callback));
    }

    pub fn value(&mut self) -> Result<&[u8]> {
        match self.value {
            Some(ref v) => Ok(v),
            None => {
                let callback = self.callback.as_ref().ok_or(Error::NoPassword)?;
                self.value = Some(SecureVec::new(callback()?));

                Ok(self.value.as_ref().unwrap())
            }
        }
    }

    #[cfg(test)]
    pub fn set_value(&mut self, value: SecureVec<u8>) {
        self.value = Some(value);
    }
}
