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

use log::{debug, warn};
use nuts_bytes::{Reader, Writer};
use nuts_container::backend::Backend;
use nuts_container::container::Container;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{ArchiveResult, Error};
use crate::magic::magic_type;

magic_type!(Magic, "invalid userdata-magic");

#[derive(Deserialize, Serialize)]
pub struct Userdata<B: Backend> {
    magic: Magic,
    pub id: B::Id,
}

impl<B: Backend> Userdata<B> {
    fn new(id: B::Id) -> Userdata<B> {
        Userdata {
            magic: Magic::new(),
            id,
        }
    }

    pub fn create(container: &mut Container<B>) -> ArchiveResult<Userdata<B>, B> {
        if !container.userdata().is_empty() {
            return Err(Error::OverwriteUserdata);
        }

        let id = container.aquire()?;
        let userdata = Userdata::<B>::new(id);

        let mut writer = Writer::new(vec![]);

        writer.serialize(&userdata)?;
        container.update_userdata(&writer.into_target())?;

        debug!("userdata created: {:?}", userdata);

        Ok(userdata)
    }

    pub fn load(container: &mut Container<B>) -> ArchiveResult<Userdata<B>, B> {
        if container.userdata().is_empty() {
            return Err(Error::InvalidUserdata(None));
        }

        let mut reader = Reader::new(container.userdata());
        let userdata = reader.deserialize::<Userdata<B>>()?;

        debug!("userdata loaded: {:?}", userdata);

        if !reader.as_ref().is_ascii() {
            warn!("there are still some unread bytes");
        }

        Ok(userdata)
    }
}

impl<B: Backend> fmt::Debug for Userdata<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Userdata").field("id", &self.id).finish()
    }
}
