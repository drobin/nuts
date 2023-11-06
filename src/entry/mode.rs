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

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

const MASK_TYPE: u16 = 0x0600;
const TYPE_FILE: u16 = 0x0000;
const TYPE_DIR: u16 = 0x0200;
const TYPE_SYMLINK: u16 = 0x0400;

const MASK_USR_R: u16 = 0x0001;
const MASK_USR_W: u16 = 0x0002;
const MASK_USR_X: u16 = 0x0004;
const MASK_GRP_R: u16 = 0x0008;
const MASK_GRP_W: u16 = 0x0010;
const MASK_GRP_X: u16 = 0x0020;
const MASK_OTH_R: u16 = 0x0040;
const MASK_OTH_W: u16 = 0x0080;
const MASK_OTH_X: u16 = 0x0100;

const DEFAULT_ACCESS_RIGHTS: u16 =
    MASK_USR_R | MASK_USR_W | MASK_USR_X | MASK_GRP_R | MASK_GRP_X | MASK_OTH_R | MASK_OTH_X;

#[derive(Clone, Copy, PartialEq)]
pub enum Group {
    User,
    Group,
    Other,
}

/// Bitmask encoding entry type & access rights.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) struct Mode(u16);

impl Mode {
    pub fn file() -> Mode {
        Mode(TYPE_FILE | DEFAULT_ACCESS_RIGHTS)
    }

    pub fn directory() -> Mode {
        Mode(TYPE_DIR | DEFAULT_ACCESS_RIGHTS)
    }

    pub fn symlink() -> Mode {
        Mode(TYPE_SYMLINK | DEFAULT_ACCESS_RIGHTS)
    }

    /// Tests whether this `Mode` instance represents a file.
    pub fn is_file(&self) -> bool {
        self.0 & MASK_TYPE == TYPE_FILE
    }

    /// Tests whether this `Mode` instance represents a directory.
    pub fn is_directory(&self) -> bool {
        self.0 & MASK_TYPE == TYPE_DIR
    }

    /// Tests whether this `Mode` instance represents a symlink.
    pub fn is_symlink(&self) -> bool {
        self.0 & MASK_TYPE == TYPE_SYMLINK
    }

    /// Tests whether a member of the given `group` has read access.
    pub fn can_read(&self, group: Group) -> bool {
        self.0 & Self::read_mask(group) > 0
    }

    /// Updates the read access attribute.
    ///
    /// If `readable` is set to `true`, a member of the given `group` becomes
    /// read access. If set to `false`, the read access is revoked.
    pub fn set_readable(&mut self, group: Group, readable: bool) {
        self.update_mask(readable, Self::read_mask(group));
    }

    /// Tests whether a member of the given `group` has write access.
    pub fn can_write(&self, group: Group) -> bool {
        self.0 & Self::write_mask(group) > 0
    }

    /// Updates the write access attribute.
    ///
    /// If `writable` is set to `true`, a member of the given `group` becomes
    /// write access. If set to `false`, the write access is revoked.
    pub fn set_writable(&mut self, group: Group, writable: bool) {
        self.update_mask(writable, Self::write_mask(group));
    }

    /// Tests whether a member of the given `group` has execute access.
    pub fn can_execute(&self, group: Group) -> bool {
        self.0 & Self::execute_mask(group) > 0
    }

    /// Updates the execute access attribute.
    ///
    /// If `executable` is set to `true`, a member of the given `group` becomes
    /// execute access. If set to `false`, the execute access is revoked.
    pub fn set_executable(&mut self, group: Group, executable: bool) {
        self.update_mask(executable, Self::execute_mask(group));
    }

    fn read_mask(group: Group) -> u16 {
        match group {
            Group::User => MASK_USR_R,
            Group::Group => MASK_GRP_R,
            Group::Other => MASK_OTH_R,
        }
    }

    fn write_mask(group: Group) -> u16 {
        match group {
            Group::User => MASK_USR_W,
            Group::Group => MASK_GRP_W,
            Group::Other => MASK_OTH_W,
        }
    }

    fn execute_mask(group: Group) -> u16 {
        match group {
            Group::User => MASK_USR_X,
            Group::Group => MASK_GRP_X,
            Group::Other => MASK_OTH_X,
        }
    }

    fn update_mask(&mut self, enable: bool, mask: u16) {
        if enable {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }
}
