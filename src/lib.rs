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

mod container;
mod entry;
mod error;
mod header;
mod magic;
#[cfg(test)]
mod tests;
mod tree;
mod userdata;

use log::debug;
use nuts_container::backend::Backend;
use nuts_container::container::Container;

pub use entry::{EntryBuilder, EntryMut};
pub use error::{ArchiveResult, Error};

use crate::container::BufContainer;
use crate::tree::Tree;
use crate::userdata::Userdata;

/// Information/statistics from the archive.
#[derive(Debug)]
pub struct Info {
    /// Number of blocks allocated for the archive
    pub blocks: u64,

    /// Number of files stored in the archive
    pub files: u64,
}

pub struct Archive<B: Backend> {
    container: BufContainer<B>,
    tree_id: B::Id,
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
        let mut container = BufContainer::new(container);
        let userdata = Userdata::create(&mut container, force)?;
        let tree = Tree::<B>::new();

        tree.flush(&mut container, &userdata.id)?;

        let archive = Archive {
            container,
            tree_id: userdata.id,
            tree,
        };

        debug!("archive created, tree: {}", archive.tree_id);

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
        let mut container = BufContainer::new(container);
        let userdata = Userdata::load(&mut container)?;
        let tree = Tree::load(&mut container, &userdata.id)?;

        let archive = Archive {
            container,
            tree_id: userdata.id,
            tree,
        };

        debug!("archive opened, tree: {}", archive.tree_id);

        Ok(archive)
    }

    /// Fetches statistics/information from the archive.
    pub fn info(&self) -> Info {
        Info {
            blocks: self.tree.nblocks(),
            files: self.tree.nfiles(),
        }
    }

    /// Appends a new entry with the given `name` at the end of the archive.
    ///
    /// The method returns an [`EntryBuilder`] instance, where you are able to
    /// set some more properties for the new entry. Calling
    /// [`EntryBuilder::build()`] will finally create the entry.
    pub fn append<'a, N: AsRef<str>>(&'a mut self, name: N) -> EntryBuilder<'a, B> {
        EntryBuilder::new(
            &mut self.container,
            &self.tree_id,
            &mut self.tree,
            name.as_ref().to_string(),
        )
    }

    /// Consumes this `Archive`, returning the underlying [`Container`].
    pub fn into_container(self) -> Container<B> {
        self.container.into_container()
    }
}

impl<B: Backend> AsRef<Container<B>> for Archive<B> {
    fn as_ref(&self) -> &Container<B> {
        &self.container
    }
}
