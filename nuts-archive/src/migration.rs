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

use log::{debug, error};
use nuts_backend::{Backend, Binary};
use nuts_bytes::Reader;
use std::marker::PhantomData;

use crate::id::Id;
use crate::magic::{Magic, MAGIC};

pub struct Migration<B>(PhantomData<B>);

impl<B: Backend> nuts_container::Migration for Migration<B> {
    fn migrate_rev0(&self, userdata: &[u8]) -> Result<Vec<u8>, String> {
        let mut reader = Reader::new(userdata);

        debug!("migrating top-id from userdata");

        match reader.read::<Magic>() {
            Ok(magic) if magic != MAGIC => {
                let err = "magic mismatch, the container does not have an archive";
                error!("{}: {:?}", err, magic);
                return Err(err.to_string());
            }
            Ok(_) => {
                // ok, nothing to do
            }
            Err(err) => {
                return Err(format!("failed to read magic from userdata: {}", err));
            }
        };

        match reader.read::<Id<B>>() {
            Ok(id) => {
                debug!("top-id: {}", id);
                Ok(id.as_bytes())
            }
            Err(err) => Err(format!("failed to read id from userdata: {}", err)),
        }
    }
}

impl<B> Default for Migration<B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
