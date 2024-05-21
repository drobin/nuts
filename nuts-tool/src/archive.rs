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

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{debug, error, trace, warn};
use nuts_archive::{Archive, Group};
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use crate::backend::PluginBackend;
use crate::{say, say_err};

#[cfg(unix)]
mod unix {
    pub const S_IRUSR: u32 = 0o0000400; // Read permission, owner.
    pub const S_IWUSR: u32 = 0o0000200; // Write permission, owner.
    pub const S_IXUSR: u32 = 0o0000100; // Execute/search permission, owner.
    pub const S_IRGRP: u32 = 0o0000040; // Read permission, group.
    pub const S_IWGRP: u32 = 0o0000020; // Write permission, group.
    pub const S_IXGRP: u32 = 0o0000010; // Execute/search permission, group.
    pub const S_IROTH: u32 = 0o0000004; // Read permission, others.
    pub const S_IWOTH: u32 = 0o0000002; // Write permission, others.
    pub const S_IXOTH: u32 = 0o0000001; // Execute/search permission, others.
}

macro_rules! into_utc {
    ($metadata:ident . $type:ident ()) => {
        if let Ok(time) = $metadata.$type() {
            time.into()
        } else {
            warn!(
                "the {} time is not available on your platform",
                stringify!($type)
            );
            Utc::now()
        }
    };
}

fn changed(metadata: &Metadata) -> DateTime<Utc> {
    if cfg!(unix) {
        DateTime::from_timestamp(metadata.ctime(), 0).unwrap_or_else(|| {
            warn!(
                "could not convert epoch ctime {} into naive datetime",
                metadata.ctime()
            );

            Utc::now()
        })
    } else {
        panic!("platform currently not supported");
    }
}

fn can_read(metadata: &Metadata, group: Group) -> bool {
    if cfg!(unix) {
        let mask = match group {
            Group::User => unix::S_IRUSR,
            Group::Group => unix::S_IRGRP,
            Group::Other => unix::S_IROTH,
        };

        metadata.permissions().mode() & mask > 0
    } else {
        match group {
            Group::User => true,
            Group::Group => false,
            Group::Other => false,
        }
    }
}

fn can_write(metadata: &Metadata, group: Group) -> bool {
    if cfg!(unix) {
        let mask = match group {
            Group::User => unix::S_IWUSR,
            Group::Group => unix::S_IWGRP,
            Group::Other => unix::S_IWOTH,
        };

        metadata.permissions().mode() & mask > 0
    } else {
        match group {
            Group::User => !metadata.permissions().readonly(),
            Group::Group => false,
            Group::Other => false,
        }
    }
}

fn can_execute(metadata: &Metadata, group: Group) -> bool {
    if cfg!(unix) {
        let mask = match group {
            Group::User => unix::S_IXUSR,
            Group::Group => unix::S_IXGRP,
            Group::Other => unix::S_IXOTH,
        };

        metadata.permissions().mode() & mask > 0
    } else {
        false
    }
}

pub fn append_recursive(archive: &mut Archive<PluginBackend>, path: &Path) -> Result<()> {
    debug!("append {}", path.display());

    let metadata = match fs::symlink_metadata(path) {
        Ok(md) => md,
        Err(err) => {
            error!("{}", err);
            say_err!("! {}", path.display());
            return Ok(());
        }
    };

    if metadata.is_file() {
        let block_size = archive.as_ref().block_size() as usize;

        let mut fh = File::open(path)?;
        let mut buf = vec![0; block_size];

        let mut builder = archive.append_file(path.to_string_lossy());

        builder.set_created(into_utc!(metadata.created()));
        builder.set_changed(changed(&metadata));
        builder.set_modified(into_utc!(metadata.modified()));

        for group in [Group::User, Group::Group, Group::Other] {
            builder.set_readable(group, can_read(&metadata, group));
            builder.set_writable(group, can_write(&metadata, group));
            builder.set_executable(group, can_execute(&metadata, group));
        }

        let mut entry = builder.build()?;

        loop {
            let n = fh.read(&mut buf)?;
            trace!("{} bytes read from {}", n, path.display());

            if n > 0 {
                entry.write_all(&buf[..n])?;
            } else {
                break;
            }
        }
    } else if metadata.is_dir() {
        let mut builder = archive.append_directory(path.to_string_lossy());

        builder.set_created(into_utc!(metadata.created()));
        builder.set_changed(changed(&metadata));
        builder.set_modified(into_utc!(metadata.modified()));

        for group in [Group::User, Group::Group, Group::Other] {
            builder.set_readable(group, can_read(&metadata, group));
            builder.set_writable(group, can_write(&metadata, group));
            builder.set_executable(group, can_execute(&metadata, group));
        }

        builder.build()?;
    } else if metadata.is_symlink() {
        let target = path.read_link()?;

        let mut builder = archive.append_symlink(path.to_string_lossy(), target.to_string_lossy());

        builder.set_created(into_utc!(metadata.created()));
        builder.set_changed(changed(&metadata));
        builder.set_modified(into_utc!(metadata.modified()));

        for group in [Group::User, Group::Group, Group::Other] {
            builder.set_readable(group, can_read(&metadata, group));
            builder.set_writable(group, can_write(&metadata, group));
            builder.set_executable(group, can_execute(&metadata, group));
        }

        builder.build()?;
    }

    say!("a {}", path.display());

    if path.is_dir() {
        for entry in path.read_dir()? {
            let child = entry?.path();

            append_recursive(archive, &child)?;
        }
    }

    Ok(())
}
