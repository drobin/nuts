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

pub use error::{ArchiveResult, Error};

use crate::container::BufContainer;
use crate::tree::Tree;
use crate::userdata::Userdata;

pub struct Archive<B: Backend> {
    container: BufContainer<B>,
    tree_id: B::Id,
    _tree: Tree<B>,
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
            _tree: tree,
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
            _tree: tree,
        };

        debug!("archive opened, tree: {}", archive.tree_id);

        Ok(archive)
    }

    /// Consumes this `Archive`, returning the underlying [`Container`].
    pub fn into_container(self) -> Container<B> {
        self.container.into_container()
    }
}

impl<'a, B: Backend> AsRef<Container<B>> for Archive<B> {
    fn as_ref(&self) -> &Container<B> {
        &self.container
    }
}
