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

use crate::header::plain_secret::tests::{rev0, ErrMigration, SampleMigration};
use crate::header::HeaderError;
use crate::migrate::{MigrationError, Migrator};

#[test]
fn rev0_no_migrator() {
    let migrator = Migrator::default();
    let mut rev0 = rev0();

    rev0.migrate(&migrator).unwrap();

    assert!(rev0.top_id.is_none());
}

#[test]
fn rev0_migrated() {
    let migrator = Migrator::default().with_migration(SampleMigration);
    let mut rev0 = rev0();

    rev0.migrate(&migrator).unwrap();

    assert_eq!(rev0.top_id.unwrap().to_string(), "4711");
}

#[test]
fn rev0_inval_migration() {
    // FIXME Id of MemoryBackend is always valid
}

#[test]
fn rev0_err_migration() {
    let migrator = Migrator::default().with_migration(ErrMigration);
    let mut rev0 = rev0();

    let err = rev0.migrate(&migrator).unwrap_err();

    assert!(matches!(err, HeaderError::Migration(cause)
        if matches!(cause, MigrationError::Rev0(ref msg) if msg == "foo")));
}
