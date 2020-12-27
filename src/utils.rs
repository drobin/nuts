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

#[cfg(test)]
mod tests;

use std::io::{self, Write};
use std::ops::{Deref, DerefMut};
use std::{default, fmt};

use crate::io::IntoBinary;

pub struct SecureVec<T>
where
    T: std::default::Default,
{
    inner: Vec<T>,
}

impl<T> SecureVec<T>
where
    T: default::Default,
{
    pub fn new(vec: Vec<T>) -> SecureVec<T> {
        SecureVec { inner: vec }
    }
}

impl<T> Drop for SecureVec<T>
where
    T: default::Default,
{
    fn drop(&mut self) {
        self.inner
            .resize_with(self.inner.capacity(), Default::default);

        for elem in self.inner.iter_mut() {
            *elem = Default::default();
        }
    }
}

impl<T> Deref for SecureVec<T>
where
    T: default::Default,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SecureVec<T>
where
    T: default::Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> PartialEq<Vec<T>> for SecureVec<T>
where
    T: default::Default + PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        self.inner.eq(other)
    }
}

impl<T> PartialEq<[T]> for SecureVec<T>
where
    T: default::Default + PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.inner.eq(&other)
    }
}

impl<T> PartialEq<&[T]> for SecureVec<T>
where
    T: default::Default + PartialEq,
{
    fn eq(&self, other: &&[T]) -> bool {
        self.inner.eq(other)
    }
}

impl<T> fmt::Debug for SecureVec<T>
where
    T: fmt::Debug + default::Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<T> IntoBinary for SecureVec<T>
where
    T: fmt::Debug + default::Default + IntoBinary,
{
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        self.inner.into_binary(w)
    }
}
