// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use nuts_backend::Backend;

use crate::error::Error;
use crate::migrate::Migration;
use crate::Container;

/// A service running on top of a [`Container`].
///
/// A concrete service should implement this trait. Later, such a service can
/// be instantiated with [`Container::open_service`] resp.
/// [`Container::create_service`].
pub trait Service<B: Backend> {
    /// The migration assiciated with this service.
    type Migration: Migration + 'static;

    /// Returns `true` if the service requires a top-id.
    ///
    /// If a top-id is required, a top-id is aquired and stored in the header
    /// of the container when creating a
    /// [service-instance](Container::create_service).
    fn need_top_id() -> bool;

    /// Returns the migration assiciated with this service.
    fn migration() -> Self::Migration;
}

/// Factory used to instantiate a [service](Service).
pub trait ServiceFactory<B: Backend> {
    /// The service
    type Service: Service<B>;

    /// Error returned by [`Self::open`] and [`Self::create`].
    ///
    /// It must be possible to put an [`Error`] instance into this error.
    type Err: From<Error<B>>;

    /// Create a [service](Service) instance.
    ///
    /// Called by [`Container::create_service`] the method should create a new
    /// [`Self::Service`] instance. The created [container](Container) is
    /// passed to this method.
    fn create(container: Container<B>) -> Result<Self::Service, Self::Err>;

    /// Open a [service](Service).
    ///
    /// Called by [`Container::open_service`] the method should open an
    /// existing [`Self::Service`] instance. The opened [container](Container)
    /// is passed to this method.
    fn open(container: Container<B>) -> Result<Self::Service, Self::Err>;
}
