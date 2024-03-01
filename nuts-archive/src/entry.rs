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

pub mod immut;
pub mod mode;
pub mod r#mut;
pub(crate) mod tstamp;

use nuts_backend::Backend;
use nuts_bytes::{FromBytes, ToBytes, Writer};

use crate::entry::mode::Mode;
use crate::entry::tstamp::Timestamps;
use crate::error::ArchiveResult;
use crate::pager::Pager;

#[cfg(test)]
const HALF: u8 = 53;
#[cfg(test)]
const FULL: u8 = 106;

#[derive(Debug, FromBytes, ToBytes)]
struct Inner {
    name: String,
    mode: Mode,
    tstamps: Timestamps,
    size: u64,
}

impl Inner {
    fn new(name: String, mode: Mode) -> Inner {
        Inner {
            name,
            mode,
            tstamps: Timestamps::new(),
            size: 0,
        }
    }

    fn load<B: Backend>(pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<Inner, B> {
        let mut reader = pager.read_buf(id)?;
        let inner = reader.read()?;

        Ok(inner)
    }

    fn flush<B: Backend>(&self, pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let buf = {
            let mut writer = Writer::new(vec![]);

            writer.write(self)?;

            writer.into_target()
        };

        pager.write(id, &buf)?;

        Ok(())
    }
}

macro_rules! populate_mode_api {
    () => {
        /// Tests whether a member of the given `group` has read access.
        pub fn can_read(&self, group: crate::Group) -> bool {
            self.inner().mode.can_read(group)
        }

        /// Tests whether a member of the given `group` has write access.
        pub fn can_write(&self, group: crate::Group) -> bool {
            self.inner().mode.can_write(group)
        }

        /// Tests whether a member of the given `group` has execute access.
        pub fn can_execute(&self, group: crate::Group) -> bool {
            self.inner().mode.can_execute(group)
        }
    };

    (mut) => {
        /// Updates the read access attribute.
        ///
        /// If `readable` is set to `true`, a member of the given `group` becomes
        /// read access. If set to `false`, the read access is revoked.
        pub fn set_readable(&mut self, group: crate::Group, readable: bool) {
            self.inner_mut().mode.set_readable(group, readable)
        }

        /// Updates the write access attribute.
        ///
        /// If `writable` is set to `true`, a member of the given `group` becomes
        /// write access. If set to `false`, the write access is revoked.
        pub fn set_writable(&mut self, group: crate::Group, writable: bool) {
            self.inner_mut().mode.set_writable(group, writable)
        }

        /// Updates the execute access attribute.
        ///
        /// If `executable` is set to `true`, a member of the given `group` becomes
        /// execute access. If set to `false`, the execute access is revoked.
        pub fn set_executable(&mut self, group: crate::Group, executable: bool) {
            self.inner_mut().mode.set_executable(group, executable)
        }
    };
}

macro_rules! populate_tstamp_api {
    () => {
        /// Returns the time when the entry was appened to the archive.
        pub fn appended(&self) -> &chrono::DateTime<chrono::Utc> {
            self.inner().tstamps.appended()
        }

        /// Returns the time when the originating filesystem entry was created.
        pub fn created(&self) -> &chrono::DateTime<chrono::Utc> {
            self.inner().tstamps.created()
        }

        /// Returns the time when the originating filesystem entry was changed
        /// the last time.
        pub fn changed(&self) -> &chrono::DateTime<chrono::Utc> {
            self.inner().tstamps.changed()
        }

        /// Returns the time when the originating filesystem entry was modified
        /// the last time.
        pub fn modified(&self) -> &chrono::DateTime<chrono::Utc> {
            self.inner().tstamps.modified()
        }
    };

    (mut) => {
        populate_tstamp_api!();

        /// Updates the creation time of the archive entry.
        pub fn set_created(&mut self, created: chrono::DateTime<chrono::Utc>) {
            self.inner_mut().tstamps.set_created(created)
        }

        /// Updates the changed time of the archive entry.
        pub fn set_changed(&mut self, changed: chrono::DateTime<chrono::Utc>) {
            self.inner_mut().tstamps.set_changed(changed)
        }

        /// Updates the modification time of the archive entry.
        pub fn set_modified(&mut self, modified: chrono::DateTime<chrono::Utc>) {
            self.inner_mut().tstamps.set_modified(modified)
        }
    };
}

use {populate_mode_api, populate_tstamp_api};
