// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use crate::backend::{Backend, Options};
use crate::container::cipher::Cipher;
use crate::container::error::ContainerResult;

use super::ContainerError;

/// Options used to create a new container.
///
/// Use the [`CreateOptionsBuilder`] utility to create a `CreateOptions`
/// instance.
#[derive(Debug)]
pub struct CreateOptions<B: Backend> {
    pub(crate) backend: B::CreateOptions,
    pub(crate) cipher: Cipher,
}

/// Utility used to create a [`CreateOptions`] instance.
pub struct CreateOptionsBuilder<B: Backend>(CreateOptions<B>);

impl<B: Backend> CreateOptionsBuilder<B> {
    /// Creates a builder instance using the given [`Backend::CreateOptions`] instance.
    pub fn for_backend(options: B::CreateOptions) -> Self {
        CreateOptionsBuilder(CreateOptions {
            backend: options,
            cipher: Cipher::None,
        })
    }

    /// Use the given `cipher` for encryption.
    pub fn with_cipher(mut self, cipher: Cipher) -> Self {
        self.0.cipher = cipher;
        self
    }

    /// Creates the [`CreateOptions`] instance.
    ///
    /// Before the [`CreateOptions`] instance is created all options passed to
    /// the builder are validated.
    ///
    /// # Errors
    ///
    /// If validation has failed a [`ContainerError`] is returned.
    pub fn build(self) -> ContainerResult<CreateOptions<B>, B> {
        self.0
            .backend
            .validate()
            .map_or_else(|cause| Err(ContainerError::Backend(cause)), |()| Ok(self.0))
    }
}

/// Options used to open a container.
///
/// Use the [`OpenOptionsBuilder`] utility to create a `OpenOptions` instance.
#[derive(Debug)]
pub struct OpenOptions<B: Backend> {
    pub(crate) backend: B::OpenOptions,
}

/// Utility used to create a [`OpenOptions`] instance.
pub struct OpenOptionsBuilder<B: Backend>(OpenOptions<B>);

impl<B: Backend> OpenOptionsBuilder<B> {
    /// Creates a builder instance using the given [`Backend::OpenOptions`] instance.
    pub fn for_backend(options: B::OpenOptions) -> Self {
        OpenOptionsBuilder(OpenOptions { backend: options })
    }

    /// Creates the [`OpenOptions`] instance.
    ///
    /// Before the [`OpenOptions`] instance is created all options passed to
    /// the builder are validated.
    ///
    /// # Errors
    ///
    /// If validation has failed a [`ContainerError`] is returned.
    pub fn build(self) -> ContainerResult<OpenOptions<B>, B> {
        self.0
            .backend
            .validate()
            .map_or_else(|cause| Err(ContainerError::Backend(cause)), |()| Ok(self.0))
    }
}
