// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

//! A secure storage library.
//!
//! The _nuts_ library implements a secure storage library, where data are
//! stored in a container. The container is divide into blocks, which are
//! encrypted separately.
//!
//! The container configuration contains all information needed to open the
//! container. It is located in the first block of the container. It has a
//! public part, which contains some basic information. The second part is
//! encrypted and basically contains the keys used to encrypt/decrypt the data
//! of the container.
//!
//! ```rust
//! // The following example creates a container.
//!
//! use nuts::container::Container;
//! use nuts::types::OptionsBuilder;
//! use tempfile::tempdir;
//!
//! // We put the container (named `container`) in a temporary directory.
//! let dir = tempdir().unwrap();
//! let file = dir.path().join("container");
//!
//! // The container is closed after creating the `Container` instance.
//! // You need to create it explicity.
//! let mut container = Container::new();
//! assert!(!container.is_open());
//!
//! // You need some options to create the container. Here we take some
//! // defaults for simplicity.
//! let options = OptionsBuilder::default().build().unwrap();
//!
//! // Before creating the container using the `create()` method, you need to
//! // assign a password callback. The returned password encrypts the secret
//! // part of the header, which basically contains the keys used to encrypt
//! // the data in the container.
//! // Afterwards the container is open.
//! container.set_password_callback(|| Ok(vec![1, 2, 3]));
//! container.create(&file, options).unwrap();
//! assert!(container.is_open());
//! ```
//!
//! ```rust
//! // The following example opens an already existing container.
//!
//! use nuts::container::Container;
//! use nuts::types::OptionsBuilder;
//! use std::path::Path;
//! use tempfile::tempdir;
//!
//! // The following function creates a container located in `file` with
//! // default options.
//! fn create_container(file: &Path) {
//!   let mut container = Container::new();
//!   let options = OptionsBuilder::default().build().unwrap();
//!
//!   container.set_password_callback(|| Ok(vec![1, 2, 3]));
//!   container.create(&file, options).unwrap();
//! }
//!
//! // We put the container (named `container`) in a temporary directory.
//! let dir = tempdir().unwrap();
//! let file = dir.path().join("container");
//!
//! // Create the container.
//! create_container(&file);
//!
//! // The container is closed after creating the `Container` instance.
//! // You need to open it explicity.
//! let mut container = Container::new();
//! assert!(!container.is_open());
//!
//! // Before opening the container using the `open()` method, you need to
//! // assign a password callback. The returned password encrypts the secret
//! // part of the header, which basically contains the keys used to encrypt
//! // the data in the container.
//! // Afterwards the container is open.
//! container.set_password_callback(|| Ok(vec![1, 2, 3]));
//! container.open(&file, None).unwrap();
//! assert!(container.is_open());
//! ```
//!
//! ```rust
//! // The following example writes data into the container and reads them
//! // back.
//!
//! use nuts::container::Container;
//! use nuts::types::OptionsBuilder;
//! use std::path::Path;
//! use tempfile::tempdir;
//!
//! // We put the container (named `container`) in a temporary directory.
//! let dir = tempdir().unwrap();
//! let file = dir.path().join("container");
//!
//! // For the example we create a container with some default options.
//!
//! let mut container = Container::new();
//! let options = OptionsBuilder::default().build().unwrap();
//!
//! container.set_password_callback(|| Ok(vec![1, 2, 3]));
//! container.create(&file, options).unwrap();
//!
//! // Write 3 bytes into the block 1.
//! let nbytes = container.write(1, b"123").unwrap();
//! assert_eq!(nbytes, 3);
//!
//! // Read block #1 back.
//! let mut buf = [0; 3];
//! let nbytes = container.read(1, &mut buf).unwrap();
//! assert_eq!(nbytes, 3);
//! assert_eq!(&buf, b"123");
//!
//! ```

#[macro_use]
pub(crate) mod macros;

pub mod container;
pub mod error;
pub(crate) mod header;
pub mod io;
pub(crate) mod password;
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
