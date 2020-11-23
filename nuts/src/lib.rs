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

#[macro_use]
pub(crate) mod macros;

pub mod container;
pub mod error;
pub(crate) mod header;
pub mod io;
pub(crate) mod rand;
pub mod result;
pub mod types;
pub(crate) mod utils;

/// Major p<art of the version.
///
/// The format of the version is **\<major\>**.\<minor\>.\<step\>[-\<extension\>]
pub const VERSION_MAJOR: u32 = 0;

/// Minor part of the version.
///
/// The format of the version is \<major\>.**\<minor\>**.\<step\>[-\<extension\>]
pub const VERSION_MINOR: u32 = 1;

/// Step part of the version.
///
/// The format of the version is \<major\>.\<minor\>.**\<step\>**[-\<extension\>]
pub const VERSION_STEP: u32 = 0;

/// Extension part of the version.
///
/// The format of the version is \<major\>.\<minor\>.\<step\>[-**\<extension\>**]
pub const VERSION_EXT: &str = "snapshot";

/// Returns a `String` that represents the version of the crate.
///
/// If [`VERSION_EXT`] is not empty, then a version string of the form
///
/// `<`[`VERSION_MAJOR`]`>.<`[`VERSION_MINOR`]`>.<`[`VERSION_STEP`]`>-<`[`VERSION_EXT`]`>`
///
/// is return<ed.
///
/// If [`VERSION_EXT`] is empty, then a string in the form
///
/// `<`[`VERSION_MAJOR`]`>.<`[`VERSION_MINOR`]`>.<`[`VERSION_STEP`]`>`
///
/// is returned.
///
/// [`VERSION_MAJOR`]: constant.VERSION_MAJOR.html
/// [`VERSION_MINOR`]: constant.VERSION_MINOR.html
/// [`VERSION_STEP`]: constant.VERSION_STEP.html
/// [`VERSION_EXT`]: constant.VERSION_EXT.html
pub fn version() -> String {
    if VERSION_EXT.len() > 0 {
        format!(
            "{}.{}.{}-{}",
            VERSION_MAJOR, VERSION_MINOR, VERSION_STEP, VERSION_EXT
        )
    } else {
        format!("{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_STEP)
    }
}
