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

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use colored::*;
use log::{debug, error, trace, warn};
use nuts_archive::Archive;
use nuts_directory::DirectoryBackend;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

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
        match NaiveDateTime::from_timestamp_opt(metadata.ctime(), 0) {
            Some(naive) => TimeZone::from_utc_datetime(&Utc, &naive),
            None => {
                warn!(
                    "could not convert epoch ctime {} into naive datetime",
                    metadata.ctime()
                );

                Utc::now()
            }
        }
    } else {
        panic!("platform currently not supported");
    }
}

pub fn append_recursive(
    archive: &mut Archive<DirectoryBackend<PathBuf>>,
    path: &Path,
) -> Result<()> {
    debug!("append {}", path.display());

    let metadata = match fs::symlink_metadata(path) {
        Ok(md) => md,
        Err(err) => {
            error!("{}", err);
            println!("{}", format!("! {}", path.display()).red());
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

        builder.build()?;
    } else if metadata.is_symlink() {
        let target = path.read_link()?;

        let mut builder = archive.append_symlink(path.to_string_lossy(), target.to_string_lossy());

        builder.set_created(into_utc!(metadata.created()));
        builder.set_changed(changed(&metadata));
        builder.set_modified(into_utc!(metadata.modified()));

        builder.build()?;
    }

    println!("a {}", path.display());

    if path.is_dir() {
        for entry in path.read_dir()? {
            let child = entry?.path();

            append_recursive(archive, &child)?;
        }
    }

    Ok(())
}
