// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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
use nuts_backend::Backend;
use nuts_bytes::{FromBytes, Reader, ToBytes, Writer};
use std::fmt;
use thiserror::Error;

use crate::error::{ArchiveResult, Error};
use crate::id::Id;
use crate::magic::{validate_magic, Magic, MagicErrorFactory, MAGIC};
use crate::pager::Pager;

#[derive(Debug, Error)]
#[error("invalid userdata")]
pub struct UserdataMagicError;

impl MagicErrorFactory for UserdataMagicError {
    fn create() -> Self {
        UserdataMagicError
    }
}

// magic_type!(Magic, "invalid userdata-magic");

#[derive(FromBytes, ToBytes)]
pub struct Userdata<B: Backend> {
    #[nuts_bytes(map_from_bytes = validate_magic::<UserdataMagicError>)]
    magic: Magic,
    pub id: Id<B>,
}

impl<B: Backend> Userdata<B> {
    fn new(id: Id<B>) -> Userdata<B> {
        Userdata { magic: MAGIC, id }
    }

    pub fn create(pager: &mut Pager<B>, force: bool) -> ArchiveResult<Userdata<B>, B> {
        if !force && !pager.userdata().is_empty() {
            return Err(Error::OverwriteUserdata);
        }

        let id = pager.aquire()?;
        let userdata = Userdata::<B>::new(id);

        let mut writer = Writer::new(vec![]);

        writer.write(&userdata)?;
        pager.update_userdata(&writer.into_target())?;

        debug!("userdata created: {:?}", userdata);

        Ok(userdata)
    }

    pub fn load(pager: &mut Pager<B>) -> ArchiveResult<Userdata<B>, B> {
        if pager.userdata().is_empty() {
            return Err(Error::InvalidUserdata(None));
        }

        let mut reader = Reader::new(pager.userdata());
        let userdata = reader.read::<Userdata<B>>()?;

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
