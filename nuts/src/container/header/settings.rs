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

#[cfg(test)]
mod tests;

use nuts_backend::Backend;
use nuts_bytes::{FromBytesExt, ToBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::container::error::ContainerResult;
use crate::svec::SecureVec;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings(SecureVec);

impl Settings {
    #[cfg(test)]
    pub fn new(vec: Vec<u8>) -> Settings {
        Settings(vec.into())
    }

    pub fn from_backend<B: Backend>(settings: &B::Settings) -> ContainerResult<Settings, B> {
        let mut cursor = Cursor::new(vec![]);

        cursor.to_bytes(settings)?;

        Ok(Settings(cursor.into_inner().into()))
    }

    pub fn into_backend<B: Backend>(self) -> ContainerResult<B::Settings, B> {
        let mut cursor = Cursor::new(self.0.as_ref());
        Ok(cursor.from_bytes()?)
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for Settings {
    fn eq(&self, other: &T) -> bool {
        self.0.as_ref() == other.as_ref()
    }
}
