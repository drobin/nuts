// MIT License
//
// Copyright (c) 2024 Robin Doer
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
use thiserror::Error;

use crate::svec::SecureVec;

#[derive(Debug, Error)]
pub enum MigrationError {
    /// Failed to migrate from rev0 to rev1
    #[error("failed to migrate the revision 0 header into revision 1")]
    Rev0(String),
}

pub trait Migration {
    /// Migration of a revision 0 header.
    ///
    /// You you have a revision 0 header, then this function is called. You
    /// must extract the `top-id` from the given `userdata` record. The concept
    /// of `userdata` was removed with the revision 1. The `top-id` is directly
    /// stored in the header.
    ///
    /// On success the extracted `top-id` should be returned, otherwise return
    /// an error-description.
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<(u32, Vec<u8>), String>;
}

#[derive(Default)]
pub struct Migrator<'a>(Option<Box<dyn Migration + 'a>>);

impl<'a> Migrator<'a> {
    pub fn with_migration<M: 'a + Migration>(mut self, migration: M) -> Self {
        self.0 = Some(Box::new(migration));
        self
    }

    pub fn migrate_rev0(
        &self,
        userdata: &[u8],
    ) -> Result<Option<(u32, SecureVec)>, MigrationError> {
        if let Some(migration) = self.0.as_ref() {
            match migration.migrate_rev0(userdata) {
                Ok((sid, top_id)) => Ok(Some((sid, top_id.into()))),
                Err(err) => Err(MigrationError::Rev0(err)),
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a> fmt::Debug for Migrator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Migrator")
            .field(&self.0.as_ref().map(|_| "..."))
            .finish()
    }
}
