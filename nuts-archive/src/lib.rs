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
//! use nuts_archive::ArchiveFactory;
//! use nuts_container::{Cipher, Container, CreateOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend};
//! use tempfile::{Builder, TempDir};
//!
//! // Let's create an archive service (with a directory backend) in a temporary directory
//! let tmp_dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();
//! let backend_options = CreateOptions::for_path(tmp_dir);
//! let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let archive =
//!     Container::create_service::<_, ArchiveFactory>(backend_options, container_options).unwrap();
//!
//! // Fetch some information
//! let info = archive.info();
//! assert_eq!(info.blocks, 0);
//! assert_eq!(info.files, 0);
//! ```
//!
//! ## Open an existing archive
//! ```rust
//! use nuts_archive::ArchiveFactory;
//! use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempfile::{Builder, TempDir};
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     Container::create_service::<_, ArchiveFactory>(backend_options, contaner_options).unwrap();
//!
//!     dir
//! };
//!
//! // Open the archive service (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//! let archive =
//!     Container::open_service::<_, ArchiveFactory>(backend_options, container_options).unwrap();
//!
//! // Fetch some information
//! let info = archive.info();
//! assert_eq!(info.blocks, 0);
//! assert_eq!(info.files, 0);
//! ```
//!
//! ## Append an entry at the end of the archive
//!
//! ```rust
//! use nuts_archive::ArchiveFactory;
//! use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempfile::{Builder, TempDir};
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     Container::create_service::<_, ArchiveFactory>(backend_options, contaner_options).unwrap();
//!
//!     dir
//! };
//!
//! // Open the archive (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//!
//! let mut archive =
//!     Container::open_service::<_, ArchiveFactory>(backend_options, container_options).unwrap();
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
//! use nuts_archive::ArchiveFactory;
//! use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
//! use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
//! use tempfile::{Builder, TempDir};
//!
//! // This will create an empty archive in a temporary directory.
//! let tmp_dir = {
//!     let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();
//!
//!     let backend_options = CreateOptions::for_path(&dir);
//!     let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
//!         .with_password_callback(|| Ok(b"123".to_vec()))
//!         .build::<DirectoryBackend<&TempDir>>()
//!         .unwrap();
//!
//!     Container::create_service::<_, ArchiveFactory>(backend_options, container_options).unwrap();
//!
//!     dir
//! };
//!
//! // Open the archive (with a directory backend) from the temporary directory.
//! let backend_options = OpenOptions::for_path(tmp_dir);
//! let container_options = OpenOptionsBuilder::new()
//!     .with_password_callback(|| Ok(b"123".to_vec()))
//!     .build::<DirectoryBackend<TempDir>>()
//!     .unwrap();
//!
//! // Open the archive and append some entries
//! let mut archive =
//!     Container::open_service::<_, ArchiveFactory>(backend_options, container_options).unwrap();
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
mod id;
mod magic;
mod migration;
mod pager;
#[cfg(test)]
mod tests;
mod tree;

use chrono::{DateTime, Utc};
use id::Id;
use log::debug;
use nuts_backend::Backend;
use nuts_bytes::PutBytesError;
use nuts_container::{Container, Service, ServiceFactory};
use std::convert::TryInto;

pub use entry::immut::{DirectoryEntry, Entry, FileEntry, SymlinkEntry};
pub use entry::mode::Group;
pub use entry::r#mut::{DirectoryBuilder, EntryMut, FileBuilder, SymlinkBuilder};
pub use error::{ArchiveResult, Error};

use crate::entry::immut::InnerEntry;
use crate::header::Header;
use crate::migration::Migration;
use crate::pager::Pager;
use crate::tree::Tree;

fn flush_header<B: Backend>(
    pager: &mut Pager<B>,
    id: &Id<B>,
    header: &Header,
    tree: &Tree<B>,
) -> ArchiveResult<(), B> {
    fn inner<B: Backend>(
        pager: &mut Pager<B>,
        header: &Header,
        tree: &Tree<B>,
    ) -> Result<usize, nuts_bytes::Error> {
        let mut writer = pager.create_writer();
        let mut n = 0;

        n += writer.write(header)?;
        n += writer.write(tree)?;

        Ok(n)
    }

    match inner(pager, header, tree) {
        Ok(n) => {
            pager.write_buf(id)?;

            debug!("{} bytes written into header at {}", n, id);

            Ok(())
        }
        Err(err) => {
            let err: Error<B> = match err {
                nuts_bytes::Error::PutBytes(PutBytesError::NoSpace) => Error::InvalidBlockSize,
                _ => err.into(),
            };
            Err(err)
        }
    }
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
    header_id: Id<B>,
    header: Header,
    tree: Tree<B>,
}

impl<B: Backend> Archive<B> {
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
    pub fn first(&mut self) -> Option<ArchiveResult<Entry<B>, B>> {
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
    pub fn lookup<N: AsRef<str>>(&mut self, name: N) -> Option<ArchiveResult<Entry<B>, B>> {
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
    pub fn append_file<N: AsRef<str>>(&mut self, name: N) -> FileBuilder<B> {
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
    pub fn append_directory<N: AsRef<str>>(&mut self, name: N) -> DirectoryBuilder<B> {
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
    pub fn append_symlink<N: AsRef<str>, T: AsRef<str>>(
        &mut self,
        name: N,
        target: T,
    ) -> SymlinkBuilder<B> {
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

impl<B: Backend> Service<B> for Archive<B> {
    type Migration = Migration<B>;

    fn need_top_id() -> bool {
        true
    }

    fn migration() -> Migration<B> {
        Migration::default()
    }
}

#[derive(Default)]
pub struct ArchiveFactory;

impl<B: Backend> ServiceFactory<B> for ArchiveFactory {
    type Service = Archive<B>;
    type Err = Error<B>;

    fn create(container: Container<B>) -> Result<Self::Service, Self::Err> {
        let mut pager = Pager::new(container);
        let top_id = pager.top_id_or_err()?;

        let header = Header::create();
        let tree = Tree::<B>::new();

        flush_header(&mut pager, &top_id, &header, &tree)?;

        let archive = Archive {
            pager,
            header_id: top_id,
            header,
            tree,
        };

        debug!("archive created, header: {}", archive.header_id);

        Ok(archive)
    }

    fn open(container: Container<B>) -> Result<Self::Service, Self::Err> {
        let mut pager = Pager::new(container);
        let top_id = pager.top_id_or_err()?;

        let mut reader = pager.read_buf(&top_id)?;
        let header = reader.read::<Header>()?;

        header.validate_revision()?;

        let tree = reader.read::<Tree<B>>()?;

        let archive = Archive {
            pager,
            header_id: top_id,
            header,
            tree,
        };

        debug!("archive opened, header: {}", archive.header_id);

        Ok(archive)
    }
}

impl<B: Backend> AsRef<Container<B>> for Archive<B> {
    fn as_ref(&self) -> &Container<B> {
        &self.pager
    }
}
