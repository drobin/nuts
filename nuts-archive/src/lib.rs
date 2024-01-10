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

//! A storage application inspired by the `tar` tool.
//!
//! The archive is an application based on the [nuts container]. Inspired by
//! the `tar` tool you can store files, directories and symlinks in a
//! [nuts container].
//!
//! * Entries can be appended at the end of the archive.
//! * They cannot be removed from the archive.
//! * You can travere the archive from the first to the last entry in the
//!   archive.
//!
//! ## Create a new archive
//!
//! ```rust
//! use nuts_archive::Archive;
//! use nuts_container::container::{Cipher, Container, CreateOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend};
//! use tempdir::TempDir;
//!
//! // Let's create a container (with a directory backend) in a temporary directory
//! let tmp_dir = TempDir::new("nuts-archive").unwrap();
//! let backend_options = CreateOptions::for_path(tmp_dir);
//! let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let container =
//!     Container::<DirectoryBackend<TempDir>>::create(backend_options, contaner_options).unwrap();
//!
//! // Now create an archive inside the container
//! let archive = Archive::create(container, false).unwrap();
//!
//! // Fetch some information
//! let info = archive.info();
//! assert_eq!(info.blocks, 0);
//! assert_eq!(info.files, 0);
//! ```
//!
//! ## Open an existing archive
//! ```rust
//! use nuts_archive::Archive;
//! use nuts_container::container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempdir::TempDir;
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = TempDir::new("nuts-archive").unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     let container =
//!         Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options)
//!             .unwrap();
//!     Archive::create(container, false).unwrap();
//!
//!     dir
//! };
//!
//! // Open the container (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let container =
//!     Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();
//!
//! // Open the archive
//! let archive = Archive::open(container).unwrap();
//!
//! // Ferch some information
//! let info = archive.info();
//! assert_eq!(info.blocks, 0);
//! assert_eq!(info.files, 0);
//! ```
//!
//! ## Append an entry at the end of the archive
//!
//! ```rust
//! use nuts_archive::Archive;
//! use nuts_container::container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempdir::TempDir;
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = TempDir::new("nuts-archive").unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     let container =
//!         Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options)
//!             .unwrap();
//!     Archive::create(container, false).unwrap();
//!
//!     dir
//! };
//!
//! // Open the container (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let container =
//!     Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();
//!
//! // Open the archive
//! let mut archive = Archive::open(container).unwrap();
//!
//! // Append a new file entry
//! let mut entry = archive.append_file("sample file").build().unwrap();
//! entry.write_all("some sample data".as_bytes()).unwrap();
//!
//! // Append a new directory entry
//! archive
//!     .append_directory("sample directory")
//!     .build()
//!     .unwrap();
//!
//! // Append a new symlink entry
//! archive
//!     .append_symlink("sample symlink", "target")
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Loop through all entries in the archive
//!
//! ```rust
//! use nuts_archive::Archive;
//! use nuts_container::container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempdir::TempDir;
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = TempDir::new("nuts-archive").unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     let container =
//!         Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options)
//!             .unwrap();
//!     Archive::create(container, false).unwrap();
//!
//!     dir
//! };
//!
//! // Open the container (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let container =
//!     Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();
//!
//! // Open the archive and append some entries
//! let mut archive = Archive::open(container).unwrap();
//!
//! archive.append_file("f1").build().unwrap();
//! archive.append_directory("f2").build().unwrap();
//! archive.append_symlink("f3", "target").build().unwrap();
//!
//! // Go through the archive
//! let entry = archive.first().unwrap().unwrap();
//! assert!(entry.is_file());
//! assert_eq!(entry.name(), "f1");
//!
//! let entry = entry.next().unwrap().unwrap();
//! assert!(entry.is_directory());
//! assert_eq!(entry.name(), "f2");
//!
//! let entry = entry.next().unwrap().unwrap();
//! assert!(entry.is_symlink());
//! assert_eq!(entry.name(), "f3");
//!
//! assert!(entry.next().is_none());
//! ```
//!
//! [nuts container]: nuts_container

mod datetime;
mod entry;
mod error;
mod header;
mod magic;
mod pager;
#[cfg(test)]
mod tests;
mod tree;
mod userdata;

use chrono::{DateTime, Utc};
use log::debug;
use nuts_backend::Backend;
use nuts_container::container::Container;
use std::cmp;
use std::convert::TryInto;

pub use entry::immut::{DirectoryEntry, Entry, FileEntry, SymlinkEntry};
pub use entry::mode::Group;
pub use entry::r#mut::{DirectoryBuilder, EntryMut, FileBuilder, SymlinkBuilder};
pub use error::{ArchiveResult, Error};

use crate::entry::immut::InnerEntry;
use crate::entry::min_entry_size;
use crate::header::Header;
use crate::pager::Pager;
use crate::tree::Tree;
use crate::userdata::Userdata;

fn flush_header<B: Backend>(
    pager: &mut Pager<B>,
    id: &B::Id,
    header: &Header,
    tree: &Tree<B>,
) -> ArchiveResult<(), B> {
    let mut writer = pager.create_writer();
    let mut n = 0;

    n += writer.write(header)?;
    n += writer.write(tree)?;

    pager.write_buf(id)?;

    debug!("{} bytes written into header at {}", n, id);

    Ok(())
}

fn min_block_size<B: Backend>() -> usize {
    let header = Header::size();
    let tree = Tree::<B>::size();
    let entry = min_entry_size();

    let min_size = cmp::max(cmp::max(header, tree), entry);

    debug!(
        "min_block_size = {} (header: {}, tree: {}, entry: {})",
        min_size, header, tree, entry
    );

    min_size
}

/// Information/statistics from the archive.
#[derive(Debug)]
pub struct Info {
    /// Time when the archive was created
    pub created: DateTime<Utc>,

    /// Time when the last entry was appended
    pub modified: DateTime<Utc>,

    /// Number of blocks allocated for the archive
    pub blocks: u64,

    /// Number of files stored in the archive
    pub files: u64,
}

/// The archive.
pub struct Archive<B: Backend> {
    pager: Pager<B>,
    header_id: B::Id,
    header: Header,
    tree: Tree<B>,
}

impl<B: Backend> Archive<B> {
    /// Creates a new archive in `container`.
    ///
    /// General initial information about the archive is stored in the
    /// [user data](Container::userdata) of the container. This means the
    /// archive can be easily opened again the next time it is
    /// [loaded](Self::open). This means that no user data is currently allowed
    /// to be stored in the container, otherwise it could be overwritten.
    /// Existing user data can be overwritten if the `force` flag is set to
    /// `true`.
    ///
    /// # Errors
    ///
    /// If user data of the container could be overwritten, an
    /// [`Error::OverwriteUserdata`] error will be returned.
    pub fn create(container: Container<B>, force: bool) -> ArchiveResult<Archive<B>, B> {
        if (container.block_size() as usize) < min_block_size::<B>() {
            return Err(Error::InvalidBlockSize);
        }

        let mut pager = Pager::new(container);
        let userdata = Userdata::create(&mut pager, force)?;

        let header = Header::create();
        let tree = Tree::<B>::new();

        flush_header(&mut pager, &userdata.id, &header, &tree)?;

        let archive = Archive {
            pager,
            header_id: userdata.id,
            header,
            tree,
        };

        debug!("archive created, header: {}", archive.header_id);

        Ok(archive)
    }

    /// Opens an archive from `container`.
    ///
    /// The initial information about the archive is loaded from the
    /// [user data](Container::userdata) of the container.
    ///
    /// # Errors
    ///
    /// If no user data is stored in the container, an
    /// [`Error::InvalidUserdata(None)`](Error::InvalidUserdata) error is
    /// returned; if it does not contain valid archive information, an
    /// [`Error::InvalidUserdata(Some(...))`](Error::InvalidUserdata) error is
    /// returned.
    pub fn open(container: Container<B>) -> ArchiveResult<Archive<B>, B> {
        if (container.block_size() as usize) < min_block_size::<B>() {
            return Err(Error::InvalidBlockSize);
        }

        let mut pager = Pager::new(container);
        let userdata = Userdata::load(&mut pager)?;

        let mut reader = pager.read_buf(&userdata.id)?;

        let header = reader.read::<Header>()?;
        let tree = reader.read::<Tree<B>>()?;

        let archive = Archive {
            pager,
            header_id: userdata.id,
            header,
            tree,
        };

        debug!("archive opened, header: {}", archive.header_id);

        Ok(archive)
    }

    /// Fetches statistics/information from the archive.
    pub fn info(&self) -> Info {
        Info {
            created: self.header.created,
            modified: self.header.modified,
            blocks: self.tree.nblocks(),
            files: self.header.nfiles,
        }
    }

    /// Returns the first entry in the archive.
    ///
    /// Next, you can use [`Entry::next()`] to traverse through the archive.
    ///
    /// If the archive is empty, [`None`] is returned.
    pub fn first<'a>(&'a mut self) -> Option<ArchiveResult<Entry<'a, B>, B>> {
        match InnerEntry::first(&mut self.pager, &mut self.tree) {
            Some(Ok(inner)) => Some(inner.try_into()),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }

    /// Searches for an entry with the given `name`.
    ///
    /// It scans the whole archive and returns the first entry which has the
    /// given name wrapped into a [`Some`]. If no such entry exists, [`None`]
    /// is returned.
    pub fn lookup<'a, N: AsRef<str>>(
        &'a mut self,
        name: N,
    ) -> Option<ArchiveResult<Entry<'a, B>, B>> {
        let mut entry_opt = self.first();

        loop {
            match entry_opt {
                Some(Ok(entry)) => {
                    if entry.name() == name.as_ref() {
                        return Some(Ok(entry));
                    }

                    entry_opt = entry.next();
                }
                Some(Err(err)) => return Some(Err(err)),
                None => break,
            }
        }

        None
    }

    /// Appends a new file entry with the given `name` at the end of the
    /// archive.
    ///
    /// The method returns a [`FileBuilder`] instance, where you are able to
    /// set some more properties for the new entry. Calling
    /// [`FileBuilder::build()`] will finally create the entry.
    pub fn append_file<'a, N: AsRef<str>>(&'a mut self, name: N) -> FileBuilder<'a, B> {
        FileBuilder::new(
            &mut self.pager,
            &self.header_id,
            &mut self.header,
            &mut self.tree,
            name.as_ref().to_string(),
        )
    }

    /// Appends a new directory entry with the given `name` at the end of the
    /// archive.
    ///
    /// The method returns a [`DirectoryBuilder`] instance, where you are able
    /// to set some more properties for the new entry. Calling
    /// [`DirectoryBuilder::build()`] will finally create the entry.
    pub fn append_directory<'a, N: AsRef<str>>(&'a mut self, name: N) -> DirectoryBuilder<'a, B> {
        DirectoryBuilder::new(
            &mut self.pager,
            &self.header_id,
            &mut self.header,
            &mut self.tree,
            name.as_ref().to_string(),
        )
    }

    /// Appends a new symlink entry with the given `name` at the end of the
    /// archive.
    ///
    /// The symlink points to the given `target` name.
    ///
    /// The method returns a [`SymlinkBuilder`] instance, where you are able to
    /// set some more properties for the new entry. Calling
    /// [`SymlinkBuilder::build()`] will finally create the entry.
    pub fn append_symlink<'a, N: AsRef<str>, T: AsRef<str>>(
        &'a mut self,
        name: N,
        target: T,
    ) -> SymlinkBuilder<'a, B> {
        SymlinkBuilder::new(
            &mut self.pager,
            &self.header_id,
            &mut self.header,
            &mut self.tree,
            name.as_ref().to_string(),
            target.as_ref().to_string(),
        )
    }

    /// Consumes this `Archive`, returning the underlying [`Container`].
    pub fn into_container(self) -> Container<B> {
        self.pager.into_container()
    }
}

impl<B: Backend> AsRef<Container<B>> for Archive<B> {
    fn as_ref(&self) -> &Container<B> {
        &self.pager
    }
}
