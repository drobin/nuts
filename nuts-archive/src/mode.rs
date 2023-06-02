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

use serde::{Deserialize, Serialize};

#[cfg(target_family = "unix")]
pub mod unix {
    // pub const S_IRWXU: u32 = 0o0000700; /* RWX mask for owner */
    pub const S_IRUSR: u32 = 0o0000400; /* R for owner */
    pub const S_IWUSR: u32 = 0o0000200; /* W for owner */
    pub const S_IXUSR: u32 = 0o0000100; /* X for owner */

    // pub const S_IRWXG: u32 = 0o0000070; /* RWX mask for group */
    pub const S_IRGRP: u32 = 0o0000040; /* R for group */
    pub const S_IWGRP: u32 = 0o0000020; /* W for group */
    pub const S_IXGRP: u32 = 0o0000010; /* X for group */

    // pub const S_IRWXO: u32 = 0o0000007; /* RWX mask for other */
    pub const S_IROTH: u32 = 0o0000004; /* R for other */
    pub const S_IWOTH: u32 = 0o0000002; /* W for other */
    pub const S_IXOTH: u32 = 0o0000001; /* X for other */

    // pub const S_IFDIR: u32 = 0o0040000; /* directory */
    // pub const S_IFREG: u32 = 0o0100000; /* regular */
    // pub const S_IFLNK: u32 = 0o0120000; /* symbolic link */
}

const MASK_RUSR: u32 = 0x00000004;
const MASK_WUSR: u32 = 0x00000002;
const MASK_XUSR: u32 = 0x00000001;
const MASK_RGRP: u32 = 0x00000020;
const MASK_WGRP: u32 = 0x00000010;
const MASK_XGRP: u32 = 0x00000008;
const MASK_ROTH: u32 = 0x00000100;
const MASK_WOTH: u32 = 0x00000080;
const MASK_XOTH: u32 = 0x00000040;

const REG_ENTRY: u32 = 1;
const DIR_ENTRY: u32 = 2;
const LNK_ENTRY: u32 = 3;

const DEFAULT_PERMISSIONS_FILE: u32 = MASK_RUSR | MASK_WUSR | MASK_RGRP | MASK_ROTH;
const DEFAULT_PERMISSIONS_DIRECTORY: u32 =
    DEFAULT_PERMISSIONS_FILE | MASK_XUSR | MASK_XGRP | MASK_XOTH;
const DEFAULT_PERMISSIONS_SYMLINK: u32 =
    DEFAULT_PERMISSIONS_FILE | MASK_XUSR | MASK_XGRP | MASK_XOTH;

/// The type of an archive entry.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    /// File
    File,

    /// Directory
    Directory,

    /// Symbolic link
    Symlink,
}

impl From<Mode> for Type {
    fn from(mode: Mode) -> Self {
        if mode.is_file() {
            Self::File
        } else if mode.is_symlink() {
            Self::Symlink
        } else {
            Self::Directory
        }
    }
}

/// Access right grouping.
///
/// Groups access rights encoded into a [`Mode`] by user, group and all (like
/// on Unix systems).
///
/// [`Mode`]: struct.Mode.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Group {
    /// The _user_ group.
    User,

    /// The _group_ group.
    Group,

    /// The _all_ group.
    Other,
}

impl Group {
    fn into_readable_flag(&self) -> u32 {
        match self {
            Group::User => MASK_RUSR,
            Group::Group => MASK_RGRP,
            Group::Other => MASK_ROTH,
        }
    }

    fn into_writable_flag(&self) -> u32 {
        match self {
            Group::User => MASK_WUSR,
            Group::Group => MASK_WGRP,
            Group::Other => MASK_WOTH,
        }
    }

    fn into_executable_flag(&self) -> u32 {
        match self {
            Group::User => MASK_XUSR,
            Group::Group => MASK_XGRP,
            Group::Other => MASK_XOTH,
        }
    }
}

/// The file mode of an [`Entry`].
///
/// The `Mode` encodes the filetype as well as the access rights of an [`Entry`].
///
/// [`Entry`]: struct.Entry.html
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Mode(u32);

impl Mode {
    fn new(ftype: u32, permissions: u32) -> Mode {
        Mode(((ftype & 0b11) << 16) | (permissions & 0b111111111))
    }

    #[cfg(test)]
    pub fn from_mask(mode: u32) -> Mode {
        let ftype = (mode >> 16) & 0b11;
        let permissions = mode & 0b111111111;

        Self::new(ftype, permissions)
    }

    pub fn from_ftype(ftype: Type) -> Mode {
        match ftype {
            Type::File => Self::file(),
            Type::Directory => Self::directory(),
            Type::Symlink => Self::symlink(),
        }
    }

    pub fn file() -> Mode {
        Self::new(REG_ENTRY, DEFAULT_PERMISSIONS_FILE)
    }

    pub fn directory() -> Mode {
        Self::new(DIR_ENTRY, DEFAULT_PERMISSIONS_DIRECTORY)
    }

    pub fn symlink() -> Mode {
        Self::new(LNK_ENTRY, DEFAULT_PERMISSIONS_SYMLINK)
    }

    fn ftype_num(&self) -> u32 {
        (self.0 >> 16) & 0b11
    }

    pub fn is_file(&self) -> bool {
        self.ftype_num() == REG_ENTRY
    }

    pub fn is_directory(&self) -> bool {
        self.ftype_num() == DIR_ENTRY
    }

    pub fn is_symlink(&self) -> bool {
        self.ftype_num() == LNK_ENTRY
    }

    pub fn is_readable(&self, group: Group) -> bool {
        self.0 & group.into_readable_flag() > 0
    }

    pub fn set_readable(&mut self, group: Group, value: bool) {
        if value {
            self.0 |= group.into_readable_flag();
        } else {
            self.0 &= !group.into_readable_flag();
        }
    }

    pub fn is_writable(&self, group: Group) -> bool {
        self.0 & group.into_writable_flag() > 0
    }

    pub fn set_writable(&mut self, group: Group, value: bool) {
        if value {
            self.0 |= group.into_writable_flag();
        } else {
            self.0 &= !group.into_writable_flag();
        }
    }

    pub fn is_executable(&self, group: Group) -> bool {
        self.0 & group.into_executable_flag() > 0
    }

    pub fn set_executable(&mut self, group: Group, value: bool) {
        if value {
            self.0 |= group.into_executable_flag();
        } else {
            self.0 &= !group.into_executable_flag();
        }
    }
}
